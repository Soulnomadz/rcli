use clap::Parser;   // 提供Opts::parse()
use rcli::process_csv;
use rcli::{Opts, SubCommand};

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
    }
    Ok(())
}


