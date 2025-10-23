use clap::Parser;

#[derive(Debug,Parser)]
#[command(name = "rcli",version,author, about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug,Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to onther formats")]
    Csv(CsvOpts),
}

#[derive(Debug,Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = validate_path_exists)]
    pub input: String,
    #[arg(short, long,default_value = "output.json")]
    pub output: String,
    #[arg(long,default_value_t = true)]
    pub header: bool,
    #[arg(short, long,default_value_t = ',')]
    pub delimiter: char,
}

fn validate_path_exists(path: &str) -> Result<String, String> {
    if std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("Path '{}' does not exist.", path))
    }
}