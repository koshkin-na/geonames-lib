//! lib for [geonames.org](https://www.geonames.org/)
//! ## Basic usage
//!
//! ```no_run
//! use std::fs::File;
//! use std::io::{BufRead, BufReader};
//! use geonames_lib::model::GeoName;
//!
//! for line in BufReader::new(File::open("allCountries.txt").unwrap()).lines() {
//!     let geo_name = GeoName::deserialize_from_string(&line.unwrap()).unwrap();
//!     println!("{:#?}", geo_name);
//! }
//! ```

pub mod model;
pub mod err;







