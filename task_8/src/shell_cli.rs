use std::{
    env::{current_dir, set_current_dir},
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

pub struct Shell;
impl Shell {
    pub fn run() {
        loop {
            let mut piped = false;
            let current_dir = current_dir().unwrap();
            print!("~{}$ ", current_dir.to_string_lossy());
            stdout().flush().unwrap();
            let mut input = String::new();
            //читаем из консоли
            stdin().read_line(&mut input).unwrap();
            //разбиваем строку по |
            let cmds = input.trim().split("|").collect::<Vec<&str>>();
            if cmds.len() > 1 {
                piped = true
            }
            let mut cmds = cmds.iter().peekable();
            let mut prev_cmd = None;
            //итерируемся
            while let Some(pipe_cmd) = cmds.next() {
                let mut parts = pipe_cmd.split_whitespace();
                let cmd = parts.next();
                let args = parts;
                match cmd {
                    Some("cd") => {
                        if let Err(err) =
                            set_current_dir(Path::new(&args.collect::<Vec<&str>>().join("")))
                        {
                            eprintln!("Error while change directory: {}", err)
                        }
                    }
                    Some("pwd") => {
                        println!("Path to current directory: {:?}", current_dir);
                    }
                    Some("echo") => {
                        println!("{:?}", args.collect::<Vec<&str>>().join(" "));
                    }
                    Some("kill") => {
                        let cmd = Command::new("kill").args(args).status();
                        match cmd {
                            Ok(status) => {
                                if status.success() {
                                    println!("The process was killed successfully");
                                } else {
                                    eprintln!(
                                        "Killing the process was done with an error :{}",
                                        status
                                    )
                                }
                            }
                            Err(err) => {
                                eprintln!("Error run kill command :{}", err)
                            }
                        }
                    }
                    Some("ps") => {
                        if piped {
                            let cmd = Command::new("ps").arg("aux").stdout(Stdio::piped()).spawn();
                            match cmd {
                                Ok(child) => {
                                    prev_cmd = Some(child);
                                }
                                Err(err) => {
                                    eprintln!("Error run ps command :{}", err)
                                }
                            }
                        } else {
                            let cmd = Command::new("ps").arg("-eo").arg("pid,time,etime").status();
                            match cmd {
                                Ok(status) => {
                                    if !status.success() {
                                        eprintln!("Ps command status :{}", status)
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Error run ps command :{}", err)
                                }
                            }
                        }
                    }

                    Some("exit") => {
                        return;
                    }
                    _ => {
                        if let Some(cmd) = cmd {
                            if piped {
                                let stdin = prev_cmd.map_or(Stdio::inherit(), |out: Child| {
                                    Stdio::from(out.stdout.unwrap())
                                });
                                let stdout = if cmds.peek().is_some() {
                                    Stdio::piped()
                                } else {
                                    Stdio::inherit()
                                };
                                let out = Command::new(cmd)
                                    .args(args)
                                    .stdin(stdin)
                                    .stdout(stdout)
                                    .spawn();
                                match out {
                                    Ok(child) => prev_cmd = Some(child),
                                    Err(err) => {
                                        println!("Error run {} command: {}", cmd, err);
                                        prev_cmd = None;
                                    }
                                }
                            } else {
                                let command = Command::new(cmd).args(args).status();
                                match command {
                                    Ok(status) => {
                                        if !status.success() {
                                            eprintln!("{} command status :{}", cmd, status)
                                        }
                                    }
                                    Err(err) => {
                                        println!("Error run {} command: {}", cmd, err);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(mut cmd) = prev_cmd {
                cmd.wait().expect("Error while run final command");
            }
        }
    }
}
