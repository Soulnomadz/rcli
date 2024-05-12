use rcli::{Opts, CmdExector};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}


