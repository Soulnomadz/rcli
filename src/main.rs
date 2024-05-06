use clap::Parser;   // 提供Opts::parse()
use rcli::{process_csv, process_genpass, process_decode, process_encode};
use rcli::{Opts, SubCommand, Base64SubCommand, TextSubCommand};

fn main() -> anyhow::Result<()>{
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
            process_genpass(opt.length, opt.uppercase, opt.lowercase, opt.numbers, opt.symbols)?;
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
                println!("{:?}", opt);
            }
            TextSubCommand::Verify(opt) => {
                println!("{:?}", opt);
            }
            // _ => println!("something is not supported"),
        }
    }
    Ok(())
}


