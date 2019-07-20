//! Read, writes and uses [World Files](https://en.wikipedia.org/wiki/World_file) for
//! [georeferenced images](https://en.wikipedia.org/wiki/Georeferencing).
//!
//! ## Example
//!
//! A world file can be created from a string, or read from a file with
//! `WorldFile::from_path(&path)`:
//! ```
//! use world_image_file::WorldFile;
//! let contents = "32.0\n0.0\n0.0\n-32.0\n691200.0\n4576000.0\n";
//! let w = WorldFile::from_string(&contents).unwrap();
//! ```
//!
//! Coordinates can be converted from image pixel to 'world' coordinates, and vice-versa.
//! ```
//! # use world_image_file::WorldFile;
//! # let contents = "32.0\n0.0\n0.0\n-32.0\n691200.0\n4576000.0\n";
//! # let w = WorldFile::from_string(&contents).unwrap();
//! assert_eq!(w.image_to_world((171., 343.)), (696672., 4565024.));
//! assert_eq!(w.world_to_image((696672., 4565024.)), (171., 343.));
//! ````
//! Pixel coordinates can be fractional. `(10.0, 2.0)` refers to the top left of pixel (10, 2).
//! `(10.5, 2.5)` is in the middle of pixel (10, 2).
//!
//! World Files do not store any SRID/spatial reference system (SRS)/coordinate reference system
//! (CRS) data. World Files were originally defined by
//! [ESRI](https://support.esri.com/en/technical-article/000002860).
//!
//! Currently `Result<WorldFile, ()>` is returned (i.e. all errors are flatted to `()`), but this
//! may change to be more descriptive, and would not be seen as a breaking change.
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// A World File
///
/// See the [module top level documention](./index.html)
#[derive(Debug, PartialEq)]
pub struct WorldFile {
    pub x_scale: f64,
    pub y_scale: f64,

    pub x_skew: f64,
    pub y_skew: f64,

    pub x_coord: f64,
    pub y_coord: f64,
}

impl WorldFile {
    /// Open a world file from a path
    pub fn from_path(p: impl AsRef<Path>) -> Result<Self, ()> {
        let p: &Path = p.as_ref();
        let mut f = File::open(p).or(Err(()))?;
        Self::from_reader(&mut f)
    }

    /// Open a world file from something that can `Read`
    pub fn from_reader(mut r: impl Read) -> Result<Self, ()> {
        let mut s = String::new();
        r.read_to_string(&mut s).or(Err(()))?;
        Self::from_string(&s)
    }

    /// Open a world file from a raw string file content
    pub fn from_string(s: impl AsRef<str>) -> Result<Self, ()> {
        let s: &str = s.as_ref();
        let lines: Vec<&str> = s.lines().collect();
        let x_scale = lines.get(0).and_then(|s| s.parse().ok()).ok_or(())?;
        let y_skew = lines.get(1).and_then(|s| s.parse().ok()).ok_or(())?;
        let x_skew = lines.get(2).and_then(|s| s.parse().ok()).ok_or(())?;
        let y_scale = lines.get(3).and_then(|s| s.parse().ok()).ok_or(())?;
        let x_coord = lines.get(4).and_then(|s| s.parse().ok()).ok_or(())?;
        let y_coord = lines.get(5).and_then(|s| s.parse().ok()).ok_or(())?;
        assert!(x_scale != 0.);
        assert!(y_scale != 0.);

        Ok(WorldFile {
            x_scale,
            y_scale,
            x_skew,
            y_skew,
            x_coord,
            y_coord,
        })
    }

    /// Convert this world file to a raw string
    pub fn to_string(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n",
            self.x_scale, self.y_skew, self.x_skew, self.y_scale, self.x_coord, self.y_coord
        )
    }

    /// Write this world file to this `Write` object
    pub fn write_to_writer(&self, mut w: impl Write) {
        write!(
            w,
            "{}\n{}\n{}\n{}\n{}\n{}\n",
            self.x_scale, self.y_skew, self.x_skew, self.y_scale, self.x_coord, self.y_coord
        )
        .unwrap();
    }

    /// Write this world file to a path
    pub fn write_to_path(&self, p: impl AsRef<Path>) {
        let mut f = File::create(p).unwrap();
        self.write_to_writer(&mut f);
    }

    /// Convert image coordinates to world coordinates.
    pub fn image_to_world(&self, image_x_y: impl Into<(f64, f64)>) -> (f64, f64) {
        let x_y = image_x_y.into();
        let x = x_y.0;
        let y = x_y.1;

        (
            self.x_scale * x + self.x_skew * y + self.x_coord,
            self.y_skew * x + self.y_scale * y + self.y_coord,
        )
    }

    /// Convert world coordinates to image coordinates
    pub fn world_to_image(&self, world_x_y: impl Into<(f64, f64)>) -> (f64, f64) {
        let x_y = world_x_y.into();
        let x = x_y.0;
        let y = x_y.1;

        let a = self.x_scale;
        let b = self.x_skew;
        let c = self.x_coord;
        let d = self.y_skew;
        let e = self.y_scale;
        let f = self.y_coord;

        let det = a * e - b * d; // determinate
        assert!(det != 0.);

        (
            (x * e - b * y + b * f - c * e) / det,
            (-x * d + a * y - a * f - c * d) / det,
        )
    }
}

impl fmt::Display for WorldFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let test = "32.0\n0.0\n0.0\n-32.0\n691200.0\n4576000.0\n";
        let w = WorldFile::from_string(&test).unwrap();
        assert_eq!(w.image_to_world((171., 343.)), (696672., 4565024.));
        assert_eq!(w.world_to_image((696672., 4565024.)), (171., 343.));
        let p = (100., 200.);
        assert_eq!(w.image_to_world(w.world_to_image(p)), p);
    }
}
