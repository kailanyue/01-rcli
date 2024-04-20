use anyhow::Result;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: SubCommand,
}

/// 1.此处的 csv 就是 subcommand 也就是输入的参数
#[derive(Debug, Parser)]
enum SubCommand {
    #[command()]
    Csv(CsvOpts),
}

/// 1.default_value 和 default_value_t 的区别
/// 2.short 和 long 的区别，以及 header 为什么没有 short

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.csv")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    match Path::new(filename).exists() {
        true => Ok(filename.into()),
        false => Err("Input file not found"),
    }
}

/// 1.可以使用 #[serde(rename_all = "PascalCase")] 来自动实现字段名和属性名的映射
/// 2.也可以使用 #[serde(rename = "Kit Number")] 来实现字段名和属性名的映射
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

/// `cargo run -- csv  -i ./assets/juventus.csv` 测试命令
/// `cargo run -- csv  -i ./assets/juventus.csv -o ./assets/juventus.json` 测试命令
fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();

            let players = reader
                .deserialize()
                .map(|result| result.unwrap())
                .collect::<Vec<Player>>();

            let json = serde_json::to_string_pretty(&players)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_is_exist() {
        let result = verify_input_file("assets/juventus.csv").unwrap();
        assert_eq!(result, "assets/juventus.csv");
    }

    #[test]
    fn test_file_is_not_exist() {
        let result = verify_input_file("assets/not_exist.csv").unwrap_err();
        assert_eq!(result, "Input file not found");
    }
}
