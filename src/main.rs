use clap::Parser;   // 提供Opts::parse()
use rcli::{process_csv, process_genpass, process_decode, process_encode};
use rcli::{Opts, SubCommand, Base64SubCommand};

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
        SubCommand::Base64(opt) => match opt {
            Base64SubCommand::Encode(opt) => {
                // println!("encode: {:?}", opt);
                process_encode(&opt.input, opt.format)?
            }
            Base64SubCommand::Decode(opt) => {
                // println!("decode: {:?}", opt);
                process_decode(&opt.input, opt.format)?
            }
        }
    }
    Ok(())
}


