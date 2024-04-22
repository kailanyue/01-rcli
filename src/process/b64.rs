use std::{fs::File, io::Read};

use crate::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encoded);
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();

    reader.read_to_string(&mut buf)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf.trim())?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf.trim())?,
    };

    // TODO: decoded data might not be string (but for this example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(decoded)
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "fixtures/hello_world.txt";
        let format = Base64Format::Standard;

        let encoded = process_encode(input, format).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQK");
    }

    #[test]
    fn test_process_encode1() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/hello_world_encode.txt";
        let format = Base64Format::Standard;

        let decoded = process_decode(input, format).unwrap();
        assert_eq!(decoded, "hello world");
    }
}
