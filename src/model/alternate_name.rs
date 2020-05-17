use serde::{Deserialize, Serialize};
use crate::err::{DeserializeErr};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AlternateName {
    /// the id of this alternate name, int
    pub alternate_name_id: i32,
    /// geonameId referring to id in table 'geoname', int
    pub geo_name_id: i32,
    /// iso 639 language code 2- or 3-characters; 4-characters 'post' for postal codes and 'iata','icao' and faac for airport codes, fr_1793 for French Revolution names,  abbr for abbreviation, link to a website (mostly to wikipedia), wkdt for the wikidataid, varchar(7)
    pub isolanguage: Option<String>,
    /// alternate name or name variant, varchar(400)
    pub alternate_name: String,
    /// '1', if this alternate name is an official/preferred name
    pub is_preferred_name: bool,
    /// '1', if this is a short name like 'California' for 'State of California'
    pub is_short_name: bool,
    /// '1', if this alternate name is a colloquial or slang term. Example: 'Big Apple' for 'New York'.
    pub is_colloquial: bool,
    /// '1', if this alternate name is historic and was used in the past. Example 'Bombay' for 'Mumbai'.
    pub is_historic: bool,
    // from period when the name was used
    //pub from: i32,
    // to period when the name was used
    // pub to: i32,
}


impl AlternateName {
    /// Receive string example
    /// ```c
    /// 1554355	5128581	en	Big Apple			1
    /// ```
    pub fn deserialize_from_string(str: &str) -> Result<Self, DeserializeErr> {
        let vec = str.split("	").collect::<Vec<&str>>();
        Ok(Self {
            alternate_name_id: vec[0].parse::<i32>()?,
            geo_name_id: vec[1].parse::<i32>()?,
            isolanguage: Some(vec[2].to_owned()).filter(|x| !x.is_empty()),
            alternate_name: vec[3].to_owned(),
            is_preferred_name: !vec[4].is_empty(),
            is_short_name: !vec[5].is_empty(),
            is_colloquial: !vec[6].is_empty(),
            is_historic: !vec[7].is_empty(),
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::model::AlternateName;

    #[test]
    fn deserialize_string_to_alternate_name() {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        for line in BufReader::new(File::open("./test-data/alternate_name_gb.txt").unwrap()).lines() {
            let _alternate_name = AlternateName::deserialize_from_string(&line.unwrap()).unwrap();
        }
    }
}