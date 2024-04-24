use crate::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use std::io::Read;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // TODO: decoded data might not be string (but for this example, we assume it is)
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "fixtures/hello_world.txt";
        let format = Base64Format::Standard;
        let encoded = process_encode(&mut get_reader(input)?, format).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQK");
        Ok(())
    }

    #[test]
    fn test_process_encode1() -> Result<()> {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(&mut get_reader(input)?, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/hello_world_encode.txt";
        let format = Base64Format::Standard;

        let decoded = process_decode(&mut get_reader(input)?, format).unwrap();
        assert_eq!(decoded, "hello world");
        Ok(())
    }
}
