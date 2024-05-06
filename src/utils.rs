use anyhow::Result;
use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_content() {
        let result = get_content("fixtures/hello_world.txt").unwrap();
        assert_eq!(String::from_utf8_lossy(&result).trim(), "hello world");
    }

    #[test]
    fn test_get_reader() {
        let mut result = get_reader("fixtures/hello_world.txt").unwrap();
        assert_eq!(result.read_to_end(&mut Vec::new()).unwrap(), 12);
    }
}
