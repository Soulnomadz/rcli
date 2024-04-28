use clap::Parser;
use super::cli::{
    csv::CsvOpts,
    genpass::GenPassOpts,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about= "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about= "Generate a random password")]
    GenPass(GenPassOpts),
    // #[command(name="base64", about= "Encode or decode base64")]
    // Base64(Base64SubCommand),
}