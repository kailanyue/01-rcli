use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

// 1.此处的 csv 就是 subcommand 也就是输入的参数
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to json or yaml")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// 1.default_value 和 default_value_t 的区别
// 2.short 和 long 的区别，以及 header 为什么没有 short

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "csv file path")]
    pub input: String,

    #[arg(short, long, help = "output file path")]
    pub output: Option<String>,

    #[arg(short, long, default_value = "json", value_parser = parse_format,help = "output format,json or yaml")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',', help = "csv delimiter")]
    pub delimiter: char,

    #[arg(long, default_value_t = true, help = "csv header")]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, help = "password length")]
    pub length: u8,

    #[arg(long, default_value = "true", help = "include uppercase letters")]
    pub uppercase: String,

    #[arg(long, default_value = "true", help = "include lowercase letters")]
    pub lowercase: String,

    #[arg(long, default_value = "false", help = "include numbers")]
    pub number: String,

    #[arg(long, default_value = "false", help = "include symbols")]
    pub symbol: String,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    match Path::new(filename).exists() {
        true => Ok(filename.into()),
        false => Err("Input file not found"),
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
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
