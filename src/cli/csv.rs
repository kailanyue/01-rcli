use super::verify_file;
use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// 1.default_value 和 default_value_t 的区别
// 2.short 和 long 的区别，以及 header 为什么没有 short

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file, help = "csv file path")]
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
