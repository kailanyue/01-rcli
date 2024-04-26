use crate::{get_reader, process_decode, process_encode, CmdExector};

use super::verify_file;
use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "input file path")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard", help = "base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "input file path")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard", help = "base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid base64 format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let ret = process_encode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let ret = process_decode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}
