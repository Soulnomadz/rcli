use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};
use regex::Regex;

use crate::CmdExector;
use crate::utils::*;
use std::io::Write;


#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name="sign", about = "Sign a text with a private/session key and return a signature")]
    Sign(JwtSignOpts),
    #[command(name="verify", about = "Verify a signature with a public/session key")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, value_parser = verify_exp)]
    pub exp: usize,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

fn verify_exp(exp: &str) -> anyhow::Result<usize> {
    let re = Regex::new(r"^(\d+)([dhmsDHMS])")?;
    
    if let Some(v) = re.captures(exp) {
        let num = v.get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()?;
        
        let unit = v.get(2)
            .unwrap()
            .as_str()
            .to_lowercase();
    
        match unit.as_str() {
            "d" => Ok(get_epoch() + num * 3600 * 3600),
            "h" => Ok(get_epoch() + num * 3600),
            "m" => Ok(get_epoch() + num * 60),
            "s" => Ok(get_epoch() + num),
            _ => Err(anyhow::anyhow!("Invalid time unit. Use d/h/m/s or D/H/M/S")),
        }
    } else {
        Err(anyhow::anyhow!("Invalid format. Use <number><unit>, e.g. 14d, 2H."))
    }
}

fn get_epoch() -> usize {
    let start = std::time::SystemTime::now();
    start.duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut secret = get_reader("fixture/jwt-secret.txt")?;
        let token = crate::generate_token(&self, &mut secret)?;

        let mut output = std::fs::File::create("fixture/jwt-token.txt")?;
        output.write_all(token.as_bytes())?;

        println!("JWT generated: {}", token);
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut secret_reader = get_reader("fixture/jwt-secret.txt")?;
        let mut token_reader = get_reader(&self.token)?;

        let verified = crate::verify_token(&mut token_reader, &mut secret_reader)?;
        println!("Verified: {}", verified);

        Ok(())
    }
}