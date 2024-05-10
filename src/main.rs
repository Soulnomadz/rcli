use clap::Parser;   // 提供Opts::parse()
// use rcli::{process_csv, process_genpass, process_decode, process_encode};
use rcli::*;
use rcli::{Opts, SubCommand, Base64SubCommand, TextSubCommand};
use rcli::utils::*;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => { 
            let output = if let Some(output) = opt.output { 
                output.clone()
            } else { 
                format!("output.{}", opt.format)
            };
            process_csv(&opt.input, &output, opt.format)?; 
        },
        SubCommand::GenPass(opt) => {
            let pass = process_genpass(opt.length, opt.uppercase, opt.lowercase, opt.numbers, opt.symbols)?;
            println!("{}", pass);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opt) => {
                // println!("encode: {:?}", opt);
                process_encode(&opt.input, opt.format)?
            }
            Base64SubCommand::Decode(opt) => {
                // println!("decode: {:?}", opt);
                process_decode(&opt.input, opt.format)?
            }
        }
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opt) => {
                // println!("{:?}", opt);
                let mut reader = get_reader(&opt.input)?;
                let key = get_content(&opt.key)?;
                let sig = process_text_sign(&mut reader, &key, opt.format)?;

                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{:?}", encoded);
            }
            TextSubCommand::Verify(opt) => {
                // println!("{:?}", opt);
                let mut reader = get_reader(&opt.input)?;
                let key = get_content(&opt.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opt.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opt.format)?;
                if verified { 
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Generate(opt) => {
                let key = process_text_key_generate(opt.format)?;
                for(k, v) in key {
                    fs::write(opt.output_path.join(k), v)?;
                }
            }
        }
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opt) => {
                println!("{:?}", opt);
                // println!("Serving at http://0.0.0.0:{}", opt.port);
                process_http_serve(opt.dir, opt.port).await?;
            }
        }
    }
    Ok(())
}


