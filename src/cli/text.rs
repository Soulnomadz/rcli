use clap::Parser;

use std::path::PathBuf;
use std::str::FromStr;
use std::fmt;

use crate::verify_file;
use crate::verify_path;
use crate::CmdExector;
use enum_dispatch::enum_dispatch;

// use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use crate::*;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generaate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
    #[command(about = "Encrypt a message with chacha20poly1305")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message with chacha20poly1305")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3"  => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _         => Err(anyhow::anyhow!("Invalid sign format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3  => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    // 待加密内容，通过控制台直接输入或从文件读取
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    // 加密密钥，通过控制台直接输入或从文件读取
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    // 待解密内容，通过控制台直接输入或从文件读取
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    // 解密密钥，通过控制台直接输入或从文件读取
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let sig = crate::process_text_sign(&mut reader, &key, self.format)?;

        let encoded = URL_SAFE_NO_PAD.encode(sig);

        println!("{:?}", encoded);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        
        let verified = crate::process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified { 
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }

        Ok(())
    }
}

impl CmdExector for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_key_generate(self.format)?;
        for(k, v) in key {
            std::fs::write(self.output_path.join(k), v)?;
        }

        Ok(())
    }
}

impl CmdExector for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;

        let encrypted = crate::process_text_encrypt(&mut reader, &key)?;
        let output = URL_SAFE_NO_PAD.encode(encrypted);
        println!("加密文本: {}", output);

        Ok(()) 
    }
}

impl CmdExector for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // 获取base64解码后的加密文本
        let encrypted = URL_SAFE_NO_PAD.decode(crate::get_content(&self.input)?)?;
        let key = crate::get_content(&self.key)?;

        let decrypted = crate::process_text_decrypt(&mut encrypted.as_slice(), &key)?;
        println!("解密文本: {}", String::from_utf8_lossy(&decrypted));

        Ok(())
    }
}