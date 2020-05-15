use derive_more::{From, Display};
use std::num::ParseIntError;


#[derive(From, Debug, Display)]
pub enum DeserializeErr {
    ParseIntErr(ParseIntError),
    ParseDecimalErr(rust_decimal::Error),
    ParseDateTimeErr(chrono::format::ParseError)
}