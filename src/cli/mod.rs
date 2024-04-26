mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

// 1.此处的 csv 就是 subcommand 也就是输入的参数
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to json or yaml")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    match filename == "-" || Path::new(filename).exists() {
        true => Ok(filename.into()),
        false => Err("File does not exist"),
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);

    match p.exists() && p.is_dir() {
        true => Ok(path.into()),
        _ => Err("Path does not exist or is not a directory"),
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
