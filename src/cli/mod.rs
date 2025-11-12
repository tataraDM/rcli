mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};
use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

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
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn validate_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Path does not exist.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_file() {
        assert_eq!(
            validate_input_file("non_existent_file.txt"),
            Err("Path does not exist.")
        );
        assert_eq!(
            validate_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(validate_input_file("-"), Ok("-".to_string()));
    }
}
