use std::io::{stdin, stdout, Write};

use clap::Parser;

#[derive(Parser)]
pub struct CutCli {
    #[arg(short = 'f', long, default_value_t = 1)]
    fields: usize,
    #[arg(short = 'd', long, default_value = "\t")]
    delimetr: String,
    #[arg(short = 's', long)]
    separated: bool,
}

impl CutCli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn run(self) {
        let mut result = Vec::new();
        loop {
            print!(">");
            stdout().flush().expect("Error while flush stdout");
            let mut buff = String::new();
            stdin()
                .read_line(&mut buff)
                .expect("Error while read from stdin");
            if buff.trim().is_empty() {
                break;
            }
            //На мой взгяд здесь нет необходимости использовать вообще флаг -s => в любом случае надо делить по TAB
            //=> всегда делать проверку на содержание delimetr в строке
            if buff.contains(&self.delimetr) {
                let data = buff
                    .split(&self.delimetr)
                    .fold(Vec::new(), |mut data, word| {
                        data.push(word.trim().to_owned());
                        data
                    });
                result.push(data);
            }
        }
        result.iter().for_each(|line| {
            if let Some(word) = line.get(self.fields - 1) {
                println!("{:?}", word);
            }
        });
    }
}
