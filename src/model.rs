use serde::{Deserialize, Serialize};
use crate::err::{DeserializeErr};
use std::str::FromStr;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, TimeZone};

/// Соответствие типов полей было проверенно на базе 05.05.2020
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GeoName {
    /// integer id of record in geonames database
    pub geonameid: i32,
    /// name of geographical point (utf8) varchar(200)
    pub name: String,
    /// name of geographical point in plain ascii characters, varchar(200)
    pub asciiname: Option<String>,
    /// alternatenames, comma separated, ascii names automatically transliterated, convenience attribute from alternatename table, varchar(10000)
    pub alternatenames: Vec<String>,
    /// latitude in decimal degrees (wgs84)
    pub latitude: Decimal,
    /// longitude in decimal degrees (wgs84)
    pub longitude: Decimal,
    /// see http://www.geonames.org/export/codes.html, char(1)
    pub feature_class: Option<String>,
    /// see http://www.geonames.org/export/codes.html, varchar(10)
    pub feature_code: Option<String>,
    /// ISO-3166 2-letter country code, 2 characters
    pub country_code: Option<String>,
    /// alternate country codes, comma separated, ISO-3166 2-letter country code, 200 characters
    pub cc2: Option<String>,
    /// fipscode (subject to change to iso code), see exceptions below, see file admin1Codes.txt for display names of this code; varchar(20)
    pub admin1_code : Option<String>,
    /// code for the second administrative division, a county in the US, see file admin2Codes.txt; varchar(80)
    pub admin2_code : Option<String>,
    /// code for third level administrative division, varchar(20)
    pub admin3_code : Option<String>,
    /// code for fourth level administrative division, varchar(20)
    pub admin4_code : Option<String>,
    /// bigint (8 byte int)
    pub population: Option<i64>,
    /// in meters, integer
    pub elevation: Option<i32>,
    /// digital elevation model, srtm3 or gtopo30, average elevation of 3''x3'' (ca 90mx90m) or 30''x30'' (ca 900mx900m) area in meters, integer. srtm processed by cgiar/ciat.
    pub dem: String,
    /// the iana timezone id (see file timeZone.txt) varchar(40)
    pub timezone: Option<String>,
    /// date of last modification in yyyy-MM-dd format
    pub modification: DateTime<Utc>,
}

impl GeoName {
    /// Receive string example
    /// ```c
    /// 1485680	Yurty	Yurty	Jurty,Yurty,Юрты	56.0498	97.6348	P	PPL	RU		20				5902		301	Asia/Irkutsk	2012-01-17
    /// ```
    pub fn deserialize_from_string(str: &str) -> Result<Self, DeserializeErr> {
        let vec = str.split("	").collect::<Vec<&str>>();
        let elevation = match vec[15].to_owned().is_empty() {
            true => None,
            false => Some(vec[15].parse::<i32>()?)
        };
        Ok(Self {
            geonameid: vec[0].parse::<i32>()?,
            name: vec[1].to_owned(),
            asciiname: Some(vec[2].to_owned()).filter(|x| !x.is_empty()),
            alternatenames: vec[3].split(",").collect::<Vec<&str>>().into_iter().map(|x| x.to_owned()).collect::<Vec<String>>(),
            latitude: Decimal::from_str(&vec[4])?,
            longitude: Decimal::from_str(&vec[5])?,
            feature_class: Some(vec[6].to_owned()).filter(|x| !x.is_empty()),
            feature_code: Some(vec[7].to_owned()).filter(|x| !x.is_empty()),
            country_code: Some(vec[8].to_owned()).filter(|x| !x.is_empty()),
            cc2: Some(vec[9].to_owned()).filter(|x| !x.is_empty()),
            admin1_code: Some(vec[10].to_owned()).filter(|x| !x.is_empty()),
            admin2_code: Some(vec[11].to_owned()).filter(|x| !x.is_empty()),
            admin3_code: Some(vec[12].to_owned()).filter(|x| !x.is_empty()),
            admin4_code: Some(vec[13].to_owned()).filter(|x| !x.is_empty()),
            population: Some(vec[14].parse::<i64>()?).filter(|x| x != &0),
            elevation,
            dem: vec[16].to_owned(),
            timezone: Some(vec[17].to_owned()).filter(|x| !x.is_empty()),
            modification: Utc.datetime_from_str(&(vec[18].to_string() + "  00:00:00 +00:00"), "%Y-%m-%d  %H:%M:%S %z")?
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::GeoName;

    #[test]
    fn deserialize_string_to_geo_name() {
        let _ = GeoName::deserialize_from_string("1485680	Yurty	Yurty	Jurty,Yurty,Юрты	56.0498	97.6348	P	PPL	RU		20				5902		301	Asia/Irkutsk	2012-01-17").unwrap();
    }
}