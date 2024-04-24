use clap::Parser;   // 提供Opts::parse()
use rcli::process_csv;
use rcli::{Opts, SubCommand};

fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => { process_csv(&opt.input, &opt.output.unwrap())? }
    }
    Ok(())
}


