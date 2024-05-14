use clap::Parser;
use super::cli::{
    csv::CsvOpts,
    genpass::GenPassOpts,
    base64::Base64SubCommand,
    text::TextSubCommand,
    http::HttpSubCommand,
    jwt::JwtSubCommand,
};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about= "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about= "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
    #[command(subcommand)]
    Jwt(JwtSubCommand),
}