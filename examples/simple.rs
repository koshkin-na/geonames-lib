use std::fs::File;
use std::io::{BufRead, BufReader};
use geonames_lib::model::{AlternateName, GeoName};



fn main() {
    for (index, line) in BufReader::new(File::open("./test-data/alternate_name_gb.txt").unwrap()).lines().enumerate() {
        let alternate_name = AlternateName::deserialize_from_string(&line.unwrap()).unwrap();
        println!("{:#?}", alternate_name);
        if index > 20 {
            break;
        }
    }

    for (index, line) in BufReader::new(File::open("./test-data/geonames_gb.txt").unwrap()).lines().enumerate() {
        let geo_name = GeoName::deserialize_from_string(&line.unwrap()).unwrap();
        println!("{:#?}", geo_name);
        if index > 20 {
            break;
        }
    }
}