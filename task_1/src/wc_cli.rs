use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use clap::{Args, Parser};
use regex::Regex;

#[derive(Args, Debug)]
#[group(multiple = false)]
struct WcOptions {
    #[arg(long, short = 'c')]
    chars: bool,
    #[arg(long, short = 'l')]
    lines: bool,
    #[arg(long, short = 'w')]
    words: bool,
}

#[derive(Parser, Debug)]
#[command(name = "wc")]
#[command(
    about = "Fork CLI 'wc' from unix",
    long_about = "CLI that is used to count the number of lines, words, and bytes in files or input from the standard stream"
)]
pub struct Cli {
    #[command(flatten)]
    options: WcOptions,
    #[arg(required = true)]
    file_path: PathBuf,
}

pub struct WcCli {
    cli: Cli,
    word_regex: Regex,
}
impl WcCli {
    pub fn new() -> Result<WcCli, Box<dyn Error>> {
        let cli = Cli::parse();
        let word_regex = Regex::new(r"[a-zA-Zа-яА-Я0-9]+")?;
        Ok(Self { cli, word_regex })
    }
    fn get_lines_iter(&self) -> io::Result<impl Iterator<Item = String>> {
        let file = File::open(&self.cli.file_path)?;
        let buff = BufReader::new(file);
        let lines = buff
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_owned());
        Ok(lines)
    }
    fn get_words_count<F>(&self, lines_iter: F) -> usize
    where
        F: Iterator<Item = String>,
    {
        lines_iter.fold(0, |count, lines| {
            let iter = self
                .word_regex
                .find_iter(&lines)
                .map(|x| x.as_str())
                .filter(|word| !word.is_empty());
            count + iter.count()
        })
    }
    fn get_chars_count<F>(&self, lines_iter: F) -> usize
    where
        F: Iterator<Item = String>,
    {
        lines_iter.fold(0, |count, lines| count + lines.chars().count())
    }
    pub fn run(self) -> io::Result<()> {
        let lines_iter = self.get_lines_iter()?;
        if self.cli.options.chars {
            let count_chars = self.get_chars_count(lines_iter);
            println!("Count chars in file: {}", count_chars);
            Ok(())
        } else if self.cli.options.lines {
            let count_lines = lines_iter.count();
            println!("Count lines in file:{count_lines}");
            Ok(())
        } else {
            let count_words = self.get_words_count(lines_iter);
            println!("Count words in file:{}", count_words);
            Ok(())
        }
    }
}
