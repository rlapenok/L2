use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    time::Instant,
};

use clap::Parser;
use cli::Cli;
use rayon::{
    iter::ParallelIterator, slice::ParallelSlice, ThreadPool, ThreadPoolBuildError,
    ThreadPoolBuilder,
};
use serde_json::json;

mod cli;

pub struct LfCli {
    cli: Cli,
}

impl LfCli {
    fn read_from_file(&self) -> io::Result<Vec<String>> {
        let path = self.cli.get_path();
        let file = File::open(path)?;
        let lines = BufReader::new(file)
            .lines()
            .filter_map(|line| match line {
                Ok(line) if !line.is_empty() => Some(line),
                _ => None,
            })
            .collect();
        Ok(lines)
    }

    pub fn new() -> Self {
        let cli = Cli::parse();
        Self { cli }
    }

    fn create_thread_pool(&self, num_workers: usize) -> Result<ThreadPool, ThreadPoolBuildError> {
        ThreadPoolBuilder::new()
            .num_threads(num_workers)
            .build()
            .inspect_err(|err| {
                println!(
                    "Error while cretae ThreadPool:{} => The LfCli will run in one thread",
                    err
                )
            })
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let lines = self.read_from_file()?;
        let num_workers = self.cli.get_num_workers();
        let lines_part = (lines.len() + num_workers - 1) / num_workers;
        let pool = self.create_thread_pool(num_workers)?;
        let start = Instant::now();
        let result = self.get_information(pool, lines, lines_part);
        let end = start.elapsed();
        let elapsed = format!("{}.{} s", end.as_secs(), end.subsec_millis());
        let information = json!({
            "elapsed":elapsed,
            "result":result
        });
        println!("{}", information);
        Ok(())
    }
    fn get_information(
        &self,
        thread_pool: ThreadPool,
        lines: Vec<String>,
        lines_part: usize,
    ) -> HashMap<char, usize> {
        let info = thread_pool.install(|| {
            lines
                .par_chunks(lines_part)
                .map(|lines| {
                    let mut local_vault = HashMap::<char, usize>::new();
                    lines.iter().for_each(|line| {
                        line.chars()
                            .filter(|char| char.is_ascii_alphabetic())
                            .for_each(|char| {
                                local_vault
                                    .entry(char.to_ascii_lowercase())
                                    .and_modify(|count| {
                                        *count += 1;
                                    })
                                    .or_insert(1);
                            });
                    });
                    local_vault
                })
                .reduce(HashMap::new, |mut global, local| {
                    local.into_iter().for_each(|(local_char, local_count)| {
                        global
                            .entry(local_char)
                            .and_modify(|count| {
                                *count += local_count;
                            })
                            .or_insert(local_count);
                    });
                    global
                })
        });
        info
    }
}
