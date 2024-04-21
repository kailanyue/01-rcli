use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

// 1.可以使用 #[serde(rename_all = "PascalCase")] 来自动实现字段名和属性名的映射
// 2.也可以使用 #[serde(rename = "Kit Number")] 来实现字段名和属性名的映射
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
// Name,Position,DOB,Nationality,Kit Number
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input).unwrap();

    let players = reader
        .deserialize()
        .map(|result| result.unwrap())
        .collect::<Vec<Player>>();

    let json = serde_json::to_string_pretty(&players)?;
    fs::write(output, json)?;

    Ok(())
}
