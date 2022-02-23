use std::error::Error;

use clap::Args;
use serde::Deserialize;

/// Change this if you have different headers in CSV file.
#[derive(Args, Debug, Deserialize, Clone, PartialEq)]
pub struct CSVStruct {
    /// English meaning of word
    pub english: String,
    /// Japanese meaning of word
    pub japanese: String,
    /// Romaji writing of japanese
    pub romaji: String,
    /// Extra notes for word, like in situation when to use and stuff like that.
    pub notes: String,
}

pub fn read_data(path: &String) -> Result<Vec<CSVStruct>, Box<dyn Error>> {
    let mut reader = match csv::Reader::from_path(&path) {
        Ok(val) => val,
        Err(e) => return Err(Box::new(e)),
    };

    let mut tmp = vec![];

    for result in reader.deserialize() {
        tmp.push(result?);
    }

    Ok(tmp)
}
