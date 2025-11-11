use std::{fmt, str::FromStr};

use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = validate_path_exists)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long,value_parser = validate_output_format,default_value = "json")]
    pub format: OutputFormat,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn validate_path_exists(path: &str) -> Result<String, &'static str> {
    if std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("Path does not exist.")
    }
}

fn validate_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            s => anyhow::bail!("Unsupported output format: {}", s),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &'static str = (*self).into();
        write!(f, "{}", s)
    }
}
