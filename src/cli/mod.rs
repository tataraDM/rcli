mod base64;
mod csv;
mod genpass;
mod text;

use std::path::{Path, PathBuf};

pub use self::{
    base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat, text::TextSignFormat,
    text::TextSubCommand,
};
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
    #[command(subcommand)]
    Text(TextSubCommand),
}

fn validate_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Path does not exist.")
    }
}

fn validate_path(path: &str) -> Result<PathBuf, &'static str> {
    let pb = Path::new(path);
    if pb.exists() && pb.is_dir() {
        Ok(path.into())
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
            validate_file("non_existent_file.txt"),
            Err("Path does not exist.")
        );
        assert_eq!(validate_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(validate_file("-"), Ok("-".to_string()));
    }
}
