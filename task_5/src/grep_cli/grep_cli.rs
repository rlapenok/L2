use std::{
    cmp::{max, min}, collections::HashSet, fs::File, io::{self, BufRead, BufReader}
};

use clap::Parser;


use super::{
    cli::Cli,
    options::{FilterOptions, GrepOption},
};

pub struct GrepCli {
    cli: Cli,
}

impl GrepCli {
    pub fn new() -> Self {
        Self { cli: Cli::parse() }
    }

    fn read_from_file(&self) -> io::Result<Vec<String>> {
        let path = self.cli.get_file_path();

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

    pub fn run(self) -> io::Result<()> {
        let lines = self.read_from_file()?;
        let (opts, filter_opts) = self.cli.get_options();
        let pattern = self.cli.get_pattern();
        let hash_set=filter_opts.into_iter().fold(HashSet::new(), |hash_set,opt|{
            let filteres_hash_set=match opt {
                FilterOptions::NotFixed=>{
                    let filtered_hash_set=lines.iter().enumerate().filter(|(_, line)|{
                        line.contains(pattern)
                    }).fold(HashSet::new(), |mut local_set,(position,_)|{
                            local_set.insert(position);
                            local_set
                    });
                    filtered_hash_set
                }
    
    
                FilterOptions::Fixed => {
                    let filtered_hash_set=lines.iter().enumerate().filter(|(_, line)|{
                        line.as_str().trim()==pattern.trim()
                    }).fold(HashSet::new(), |mut local_set,(position,_)|{
                            local_set.insert(position);
                            local_set
                    });
                    filtered_hash_set
                }
                FilterOptions::Invert => {
    
                    let filtered_hash_set=lines.iter().enumerate().filter(|(_, line)|{
                        !line.trim().contains(pattern.trim())||line.as_str().trim()!=pattern.trim()
                    }).fold(HashSet::new(), |mut local_set,(position,_)|{
                            local_set.insert(position);
                            local_set
                    });
                    filtered_hash_set
                }
                FilterOptions::Ignore=>{
                    let filtered_hash_set=lines.iter().enumerate().filter(|(_, line)|{
                        line.to_lowercase().contains(pattern.trim().to_lowercase().as_str())||line.to_lowercase()==pattern.trim().to_lowercase()
                    }).fold(HashSet::new(), |mut local_set,(position,_)|{
                            local_set.insert(position);
                            local_set
                    });
                    filtered_hash_set
                }
            };
            if hash_set.is_empty(){
                return  filteres_hash_set
            }
            hash_set.symmetric_difference(&filteres_hash_set).copied().collect()
        });
        opts.into_iter().for_each(|opt|{
            match opt {
                GrepOption::After(num)=>{
                        hash_set.iter().for_each(|position|{
                            println!("Pos {}",position);
                            let indexes=get_indexes_for_after(num, *position, lines.len());
                            println!("Start:{} End:{}",indexes.0,indexes.1);
                            println!("After the match {:?}",&lines[indexes.0..indexes.1])
                        });
                }
                GrepOption::Before(num)=>{
                    hash_set.iter().for_each(|position|{
                        let indexes=get_indexes_for_before(num, *position);
                        println!("Before the match {:?}",&lines[indexes.0..=indexes.1])
                    });
                }
                GrepOption::Context(num)=>{
                    hash_set.iter().for_each(|position|{
                        println!("Position:{}",position);
                       let indexes_before=get_indexes_for_before(num, *position);
                    let before_lines=&lines[indexes_before.0..indexes_before.1];
                    let indexes_after=get_indexes_for_after(num, *position, lines.len());
                        let end_lines=&lines[indexes_after.0..indexes_after.1];
                        println!("Before the match: {:?}  After the match: {:?}",before_lines, end_lines)
                    });

                }
                GrepOption::Count=>{
                    println!("Count: {}",&hash_set.len())
                }
                GrepOption::LineNum=>{
                    hash_set.iter().for_each(|position|{
                        println!("Line num: {}",position+1)
                    });
                }
                GrepOption::Current=>{
                    hash_set.iter().for_each(|postition|{
                        println!("Find:{}",lines[*postition])
                    });
                }
                
            }
        });
        Ok(())
    }
}


fn get_indexes_for_before(num:usize,position:usize)->(usize,usize){

    if max(0, position as i32-num as i32)>0{
        (num,position)
}else{
    (0,position)
}


}


fn get_indexes_for_after(num:usize,position:usize,len:usize)->(usize,usize){

    if position+1==len{
        (0,0)
    }else {
        if min(len, position+num)>=len{
            return (position+1,len)
        }else{
            let start_posotion=position+1;
            let end_position=position+num;
            (start_posotion,end_position)
        }
    }


}

