mod base64;
mod csv;
mod genpass;
mod text;

use std::path::Path;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    text::TextSubCommand,
};

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
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    match filename == "-" || Path::new(filename).exists() {
        true => Ok(filename.into()),
        false => Err("File does not exist"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_is_exist() {
        let result = verify_file("assets/juventus.csv").unwrap();
        assert_eq!(result, "assets/juventus.csv");
    }

    #[test]
    fn test_file_is_not_exist() {
        let result = verify_file("assets/not_exist.csv").unwrap_err();
        assert_eq!(result, "File does not exist");
    }

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
