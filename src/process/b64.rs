use anyhow::Result;
use std::io::Read;
use crate::cli::base64::*;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(input: &str, format: Base64Format) -> Result<(String)> {
    // println!("input: {}, format: {:?}", input, format);
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe  => URL_SAFE_NO_PAD.encode(&buf),
    };
    // println!("{}", encoded);
    Ok((encoded))
}


pub fn process_decode(input: &str, format: Base64Format) -> Result<(String)> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe  => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    // println!("{}", String::from_utf8(decoded)?);
    Ok(String::from_utf8(decoded)?)
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
}