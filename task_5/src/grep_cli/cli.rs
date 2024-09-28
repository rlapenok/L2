

use std::path::PathBuf;

use crate::grep_cli::options::GrepOption;
use clap::{Args, Parser};

use super::options::FilterOptions;

#[derive(Args)]
struct CliOptions {
    #[arg(short = 'A', long, default_value = None)]
    after: Option<usize>,
    #[arg(short = 'B', long, default_value = None)]
    before: Option<usize>,
    #[arg(short = 'C', long, default_value = None)]
    context: Option<usize>,
    #[arg(short = 'c', long)]
    count: bool,
    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,
    #[arg(short = 'v', long)]
    invert: bool,
    #[arg(short = 'F', long)]
    fixed: bool,
    #[arg(short = 'n', long = "line num")]
    line_num: bool,
}
#[derive(Parser)]
pub struct Cli {
    #[command(flatten)]
    options: CliOptions,
    #[arg(short = 'p', long)]
    pattern: String,
    path: PathBuf,
}

impl Cli {
    pub fn get_file_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_options(&self) -> (Vec<GrepOption>, Vec<FilterOptions>) {
        let mut current_flag=true;
        let mut not_fixed_flag=true;
        let mut opts = Vec::new();
        let mut filter_opts = Vec::new();
        if let Some(num) =self.options.after  {
            opts.push(GrepOption::After(num));
            current_flag=false;    
        }
        if let Some(num) =self.options.before  {
            opts.push(GrepOption::Before(num));    
            current_flag=false;
        }
        if let Some(num) =self.options.context  {
            opts.push(GrepOption::Context(num));    
            current_flag=false;
        }
        if self.options.count {
            opts.push(GrepOption::Count);
        }
        if self.options.ignore_case {
            filter_opts.push(FilterOptions::Ignore);
            //not_fixed_flag=false
        }
        if self.options.invert {
            filter_opts.push(FilterOptions::Invert);
            not_fixed_flag=false
        }
        if self.options.fixed {
            filter_opts.push(FilterOptions::Fixed);
            not_fixed_flag=false
        }
        if self.options.line_num{
            opts.push(GrepOption::LineNum);
        }
        if self.options.count{
            opts.push(GrepOption::Count);
        }
        if current_flag{
            opts.push(GrepOption::Current);
        }
        if not_fixed_flag{
        filter_opts.push(FilterOptions::NotFixed);
        }
        (opts, filter_opts)
    }
    pub fn get_pattern(&self) -> &str {
        &self.pattern
    }
}
