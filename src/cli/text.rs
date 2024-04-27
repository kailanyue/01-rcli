use std::{fmt, path::PathBuf, str::FromStr};

use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::{
    get_content, get_reader, parse_base64_format, process_text_decrypt, process_text_encrypt,
    process_text_key_generate, process_text_sign, process_text_verify, Base64Format, CmdExector,
};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
    #[command(about = "Encrypt a text with a public/session key")]
    Encrypt(EncryptOpts),
    #[command(about = "Decrypt a text with a private/session key")]
    Decrypt(DecryptOpts),
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file, help = "input file path")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,  help = "key file path")]
    pub key: String,
    #[arg(long, value_parser = verify_file,  help = "key file path")]
    pub nonce: String,
    #[arg(long, default_value = "standard", value_parser = parse_base64_format, help = "base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file, help = "input file path")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,  help = "key file path")]
    pub key: String,
    #[arg(long, value_parser = verify_file,  help = "key file path")]
    pub nonce: String,
    #[arg(long, default_value = "standard", value_parser = parse_base64_format, help = "text sign format")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file, help = "input file path")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file,  help = "key file path")]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format, help = "text sign format")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file, help = "input file path")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format, help = "text sign format")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format, help = "text sign format")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path, help = "input file path")]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("invalid text sign format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;

        // base64 output
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;

        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        match verified {
            true => println!("✓ Signature verified"),
            false => println!("⚠ Signature not verified"),
        }
        Ok(())
    }
}

impl CmdExector for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output_path.join(k), v).await?;
        }
        Ok(())
    }
}

impl CmdExector for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let nonce = get_content(&self.nonce)?;

        let encrypt_str = process_text_encrypt(&mut reader, &key, &nonce, self.format)?;
        println!("encrypt result:{}", encrypt_str);
        Ok(())
    }
}

impl CmdExector for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let nonce = get_content(&self.nonce)?;

        let decrypt_str = process_text_decrypt(&mut reader, &key, &nonce, self.format)?;
        println!("decrypt result:{}", decrypt_str);
        Ok(())
    }
}
