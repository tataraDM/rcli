use crate::Base64Format;
use anyhow::{Ok, Result};
use base64::{Engine, engine::general_purpose};
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(&buf),
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let decoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf.trim())?,
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.decode(buf.trim())?,
    };

    let decoded_str = String::from_utf8(decoded)?;
    println!("{}", decoded_str);
    Ok(())
}

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
