use jsonwebtoken::{encode, decode, Header, EncodingKey};
use crate::JwtSignOpts;
use std::io::Read;

pub fn generate_token(
    claims: &JwtSignOpts,
    sec_reader: &mut Box<dyn Read>,
) -> anyhow::Result<String> {
    let mut secret = Vec::new();
    sec_reader.read_to_end(&mut secret)?;
    let key = EncodingKey::from_secret(&secret);
    let token = encode(&Header::default(), claims, &key)?;

    Ok(token)
}

pub fn verify_token(
    token_reader: &mut Box<dyn Read>, 
    secet_reader: &mut Box<dyn Read>,
) -> anyhow::Result<bool> {
    let mut buf_token = Vec::new();
    token_reader.read_to_end(&mut buf_token)?;
    let token = std::str::from_utf8(&buf_token)?;

    let mut buf_secret = Vec::new();
    secet_reader.read_to_end(&mut buf_secret)?;
    let key = jsonwebtoken::DecodingKey::from_secret(&buf_secret.as_ref());
    
    let mut validation = jsonwebtoken::Validation::default();
    validation.validate_aud = false;
    // println!("validation: {:?}", validation);

    let token_data = decode::<JwtSignOpts>(token, &key, &validation);
    println!("token_data: {:?}", token_data);

    Ok(token_data.is_ok())
}