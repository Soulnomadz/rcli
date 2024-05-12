use clap::Parser;
use std::str::FromStr;
use crate::{verify_file, CmdExector};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)] 
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
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
        match s.to_lowercase().as_str() {
            "standard"  => Ok(Base64Format::Standard),
            "urlsafe"   => Ok(Base64Format::UrlSafe),
            _           => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret= crate::process_encode(&self.input, self.format)?;
        println!("{}", ret);

        Ok(())
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_decode(&self.input, self.format)?;
        println!("{}", ret);
        
        Ok(())
    }
}
