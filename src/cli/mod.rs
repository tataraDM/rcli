mod csv;
mod genpass;

use self::csv::CsvOpts;
use self::genpass::GenPassOpts;
use clap::Parser;
pub use csv::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli",version,author, about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to onther formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate ")]
    GenPass(GenPassOpts),
}
