use serde::{Deserialize, Serialize};
use crate::err::{DeserializeErr};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AdminCode {
    /// alpha 2
    pub country_code: String,
    pub code: String,
    pub name: String,
    /// reference geo_name
    pub geo_name_id: i32,
}

impl AdminCode {
    /// Receive string example
    /// ```c
    /// NL.16	Flevoland	Flevoland	3319179
    /// ```
    pub fn deserialize_from_string(str: &str) -> Result<Self, DeserializeErr> {
        let vec = str.split("	").collect::<Vec<&str>>();
        let code = vec[0].split(".").collect::<Vec<&str>>();

        Ok(Self {
            country_code: code[0].to_owned(),
            code: code[1].to_owned(),
            name: vec[1].to_owned(),
            geo_name_id: vec[3].parse::<i32>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::AdminCode;

    #[test]
    fn deserialize_string_to_admin_code() {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        for line in BufReader::new(File::open("./test-data/admin1CodesASCII.txt").unwrap()).lines() {
            let _admin_code = AdminCode::deserialize_from_string(&line.unwrap()).unwrap();
        }
    }
}