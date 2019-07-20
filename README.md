# `world_image_file`

[![Build Status](https://travis-ci.org/rory/slippy-map-tiles-rs.svg?branch=master)](https://travis-ci.org/rory/slippy-map-tiles-rs)
[![Crates.io](https://img.shields.io/crates/v/slippy-map-tiles.svg?maxAge=2592000)](https://crates.io/crates/slippy-map-tiles)
[![Documentation](https://docs.rs/slippy-map-tiles/badge.svg)](https://docs.rs/slippy-map-tiles/)

Read, writes and uses [World Files](https://en.wikipedia.org/wiki/World_file) for
[georeferenced images](https://en.wikipedia.org/wiki/Georeferencing).

## Example

A world file can be created from a string, or read from a file with
`WorldFile::from_path(&path)`:
```
use world_image_file::WorldFile;
let contents = "32.0\n0.0\n0.0\n-32.0\n691200.0\n4576000.0\n";
let w = WorldFile::from_string(&contents).unwrap();
```

Coordinates can be converted from image pixel to 'world' coordinates, and vice-versa.
```
# use world_image_file::WorldFile;
# let contents = "32.0\n0.0\n0.0\n-32.0\n691200.0\n4576000.0\n";
# let w = WorldFile::from_string(&contents).unwrap();
assert_eq!(w.image_to_world((171., 343.)), (696672., 4565024.));
assert_eq!(w.world_to_image((696672., 4565024.)), (171., 343.));
````
Pixel coordinates can be fractional. `(10.0, 2.0)` refers to the top left of pixel (10, 2).
`(10.5, 2.5)` is in the middle of pixel (10, 2).

World Files do not store any SRID/spatial reference system (SRS)/coordinate reference system
(CRS) data. World Files were originally defined by
[ESRI](https://support.esri.com/en/technical-article/000002860).

Currently `Result<WorldFile, ()>` is returned (i.e. all errors are flatted to `()`), but this
may change to be more descriptive, and would not be seen as a breaking change.

## Copyright & Licence

Copyright [GNU Affero GPL v3 (or
later)](https://www.gnu.org/licenses/agpl-3.0.en.html). See the file
[LICENCE](LICENCE)
