use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use clap::{Args, Parser};

use crate::cli::utils::Options;
#[derive(Args, Debug)]
#[group(multiple = true)]
struct SortOptions {
    #[arg(long, short = 'k', default_value_t = 0)]
    column: usize,
    #[arg(long, short = 'n')]
    number: bool,
    #[arg(long, short = 'r')]
    revers: bool,
    #[arg(long, short = 'u')]
    non_repeating: bool,
    #[arg(long, short = 'M')]
    month: bool,
    #[arg(long, short = 'b')]
    skip_white_space: bool,
    #[arg(long, short = 'c')]
    is_sort: bool,
    #[arg(long, short = 's')]
    suffix: bool,
}

#[derive(Parser, Debug)]

pub struct Cli {
    #[command(flatten)]
    options: SortOptions,
    #[arg(required = true, long, short = 'i')]
    input_path: PathBuf,
    #[arg(long, short = 'o')]
    output_path: Option<PathBuf>,
}

impl Cli {
    fn get_column_num(&self) -> usize {
        let column_number = self.options.column;
        if column_number > 0 {
            return column_number - 1;
        }
        column_number
    }
    pub fn load_file(&self) -> io::Result<Vec<String>> {
        let file = File::open(&self.input_path)?;
        let lines = BufReader::new(file)
            .lines()
            .filter_map(|line| match line {
                Ok(line) if !line.is_empty() => Some(line),
                _ => None,
            })
            .collect();
        Ok(lines)
    }
    pub fn get_options(&self) -> (usize, bool, Vec<Options>) {
        let mut options = Vec::new();
        let column_number_option = self.get_column_num();
        let skip_white_space = self.options.skip_white_space;
        if self.options.number {
            options.push(Options::Number)
        }
        if self.options.revers {
            options.push(Options::Revers);
        }
        if self.options.non_repeating {
            options.push(Options::NonRepeating);
        }
        if self.options.month {
            options.push(Options::Month);
        }
        if self.options.is_sort {
            options.push(Options::IsSorter);
        }
        if self.options.suffix {
            options.push(Options::Suffix);
        }
        println!("Options:{:?}", options);
        (column_number_option, skip_white_space, options)
    }
    pub fn get_output(&self) -> &Option<PathBuf> {
        &self.output_path
    }
}
