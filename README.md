# Rust GeoNames.org Reader #

Library for reading format geonames
https://www.geonames.org/

## Usage ##

Add this to your `Cargo.toml`:

```toml
[dependencies]
geonames-lib = "0.1"
```
Code:

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use geonames_lib::model::{GeoName, AlternateName};


fn main() {
    for (index, line) in BufReader::new(File::open("alternateNames.txt").unwrap()).lines().enumerate() {
        let alternate_name = AlternateName::deserialize_from_string(&line.unwrap()).unwrap();
        println!("{:#?}", alternate_name);
        if index > 20 {
            break;
        }
    }

    for (index, line) in BufReader::new(File::open("allCountries.txt").unwrap()).lines().enumerate() {
        let geo_name = GeoName::deserialize_from_string(&line.unwrap()).unwrap();
        println!("{:#?}", geo_name);
        if index > 20 {
            break;
        }
    }
}
```

## Contributing ##

Contributions welcome! Please fork the repository and open a pull request
with your changes.

## License ##

geonames-lib is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.


