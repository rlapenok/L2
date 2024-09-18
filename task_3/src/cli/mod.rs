use std::{collections::{HashMap, HashSet}, fs::File, io::Write};

use clap::Parser;
use sort_cli::Cli;
use utils::{get_words, is_sort_num, is_sort_string, sort_month, sort_number, Month, Options};

mod sort_cli;
mod utils;

pub struct SortCli {
    cli: Cli,
}

impl SortCli {
    pub fn new() -> Self {
        let cli = Cli::parse();
        Self { cli }
    }
    fn sort(
        &self,
        data: Vec<String>,
        opts: (usize, bool, Vec<Options>),
    ) -> Result<Vec<String>, String> {
        let suffix_vault = HashMap::from([
            ("g", (1024 * 1024 * 1024) as i64),
            ("m", (1024 * 1024) as i64),
            ("k", 1024_i64),
        ]);
        let mut need_revers = false;
        let mut is_number = false;
        let mut is_month = false;
        let mut data = get_words(data, (opts.0, opts.1));

        for opt in opts.2 {
            match opt {
                Options::Number => {
                    is_number = true;
                    data = data
                        .into_iter()
                        .filter_map(|word| word.parse::<f64>().ok().map(|num| num.to_string()))
                        .collect();
                }
                Options::Revers => {
                    need_revers = true;
                }
                Options::NonRepeating => {
                    let mut vault = HashSet::new();
                    data = data.into_iter().fold(Vec::new(), |mut data, word| {
                        if vault.insert(word.to_owned()) {
                            data.push(word);
                            data
                        } else {
                            data
                        }
                    })
                }
                Options::Month => {
                    data = data
                        .into_iter()
                        .filter_map(|word| {
                            Month::try_from(word.as_str())
                                .map(|month| month.to_string())
                                .ok()
                        })
                        .collect();
                    is_month = true;
                    is_number = true;
                }
                Options::IsSorter => {
                    if is_number {
                        let x = data
                            .iter()
                            .filter_map(|word| word.trim().parse::<f64>().ok())
                            .fold(Vec::new(), |mut vault, num| {
                                vault.push(num);
                                vault
                            });
                        if !is_sort_num(&x) {
                            return Err("Need sort".to_owned());
                        }
                    } else if !is_sort_string(data.as_slice(), opts.1) {
                        return Err("Need sort".to_owned());
                    }
                }
                Options::Suffix => {
                    data = data
                        .iter()
                        .filter_map(|word_with_suffix| {
                            let mut number = None;

                            suffix_vault.iter().for_each(|(k, v)| {
                                if word_with_suffix.to_lowercase().ends_with(k) {
                                    let new_number = &word_with_suffix
                                        [..word_with_suffix.len() - k.len()]
                                        .parse::<f64>()
                                        .ok()
                                        .map(|num| num * (*v as f64));
                                    number = *new_number;
                                }
                            });
                            number
                        })
                        .map(|num| num.to_string())
                        .collect();
                }
            }
        }
        if is_number {
            data = sort_number(data);
        }
        if is_month {
            data = sort_month(data);
        } else {
            data.sort();
        }

        if need_revers {
            data.reverse();
        }
        Ok(data)
    }
    pub fn run(self) -> Result<(), String> {
        let opts = self.cli.get_options();
        let data = self.cli.load_file().map_err(|err| err.to_string())?;
        let sorted_data = self.sort(data, opts)?;
        if let Some(out_path) = self.cli.get_output() {

            let mut file=File::options().append(true).read(true).create(true).open(out_path).map_err(|err|{
                err.to_string()
            })?;
            let sorted_data=sorted_data.join("\n");
            file.write_all(sorted_data.as_bytes()).map_err(|err|{
                err.to_string()
            })?;
        }
        for i in sorted_data {
            println!("{i}");
        }
        Ok(())
    }
}
