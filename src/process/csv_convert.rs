use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for ele in reader.records() {
        let record = ele?;

        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
