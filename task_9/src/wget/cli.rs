use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(required = true)]
    pub url: String,
    #[arg(short = 'p', default_value = "./site")]
    pub path: PathBuf,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
