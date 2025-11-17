use std::path::PathBuf;

use clap::Parser;

use super::validate_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Start HTTP server")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".", value_parser = validate_path)]
    pub addr: PathBuf,
    #[arg(short, long, default_value = "25600")]
    pub port: u16,
}
