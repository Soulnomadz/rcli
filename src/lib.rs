mod opts;
mod process;
mod cli;
pub mod utils;

pub use opts::*;
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_genpass;
pub use process::b64::*;
pub use process::text::*;
pub use process::http_serve::process_http_serve;
pub use process::jwt::*;

use std::path::{Path, PathBuf};

pub use cli::base64::Base64SubCommand;
pub use cli::text::{TextSubCommand, TextSignFormat};
pub use cli::http::HttpSubCommand;
pub use utils::*;
pub use cli::csv::*;
pub use cli::base64::*;
pub use cli::genpass::*;
pub use cli::http::*;
pub use cli::text::*;
pub use cli::jwt::*;

use enum_dispatch::enum_dispatch;

/// =================================================================
///  检验函数
/// 
pub fn verify_file(input_file: &str) -> Result<String, String> {
    if input_file == "-" || Path::new(input_file).exists() {
        Ok(input_file.into())
    } else {
        Err(format!("Input file not found: {}", input_file))
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path not found")
    }
}

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("Input file not found: *".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("Input file not found: not-exist".into()));
    }

    #[test]
    fn test_vetify_path() {
        assert_eq!(verify_path("src"), Ok(PathBuf::from("src")));
        // assert_eq!(verify_path("-"), Ok(std::io::stdin() as AsRef<PathBuf>));
    }
}
