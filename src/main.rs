use rcli::{process_csv, Opts, Subcommand};
use clap::Parser;


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    
    }
    Ok(())
}

