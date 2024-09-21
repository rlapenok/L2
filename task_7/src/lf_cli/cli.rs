use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, default_value_t = 1)]
    t: usize,
    path: PathBuf,
}
impl Cli {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_num_workers(&self) -> usize {
        self.t
    }
}
