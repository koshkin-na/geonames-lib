//! lib for [geonames.org](https://www.geonames.org/)
//! ## Basic usage
//!
//! ```no_run
//! use std::fs::File;
//! use std::io::{BufRead, BufReader};
//! use geonames_lib::model::{GeoName, AlternateName};
//!
//! fn main() {
//!     for (index, line) in BufReader::new(File::open("alternateNames.txt").unwrap()).lines().enumerate() {
//!         let alternate_name = AlternateName::deserialize_from_string(&line.unwrap()).unwrap();
//!         println!("{:#?}", alternate_name);
//!         if index > 20 {
//!             break;
//!         }
//!     }
//!     for (index, line) in BufReader::new(File::open("allCountries.txt").unwrap()).lines().enumerate() {
//!         let geo_name = GeoName::deserialize_from_string(&line.unwrap()).unwrap();
//!         println!("{:#?}", geo_name);
//!         if index > 20 {
//!             break;
//!         }
//!     }
//! }
//! ```

pub mod model;
pub mod err;







