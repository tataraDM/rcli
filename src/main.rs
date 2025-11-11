// TODO rcli csv -i input.csv -o output.json
use clap::Parser;
use rcli::{Opts, SubCommand, process_csv, process_genpass};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.number,
                opts.symbol,
            )?;
        }
    }
    Ok(())
}
