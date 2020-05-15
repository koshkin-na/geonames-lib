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
use geonames_lib::model::GeoName;

fn main() {
    for line in BufReader::new(File::open("allCountries.txt").unwrap()).lines() {
        let geo_name = GeoName::deserialize_from_string(&line.unwrap()).unwrap();
        println!("{:#?}", geo_name);
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


