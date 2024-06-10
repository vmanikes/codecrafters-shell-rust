mod shell;
mod builtin;

use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, exit};
use crate::builtin::BuiltInCommand;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    print!("{} ", shell::SHELL_SIGN);

    io::stdout().flush().expect("unable to flush stdout");

    while stdin.read_line(&mut input).is_ok() {
        let command = input.split_whitespace().collect::<Vec<&str>>();
        let args = command.clone().into_iter().skip(1).collect::<Vec<&str>>();

        if command.len() >= 1 {
            match BuiltInCommand::from_str(command[0]) {
                BuiltInCommand::Exit => {
                    handle_exit(&args);
                },
                BuiltInCommand::Echo => {
                    handle_echo(&args);
                },
                BuiltInCommand::Type => {
                    handle_type(&args);
                },
                _ => {
                    execute_command(command[0], &args);
                }
            }
        }

        input.clear();
        print!("{} ", shell::SHELL_SIGN);
        io::stdout().flush().expect("unable to flush stdout");
    }
}

fn execute_command(command: &str, args: &Vec<&str>) {
    match command {
        "cd" => {
            match args[0] {
                "~" => {
                    let _ = env::set_current_dir(env::var("HOME").unwrap());
                },
                _ => {
                    match env::set_current_dir(args[0]) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("cd: {}: No such file or directory", args[0])
                        }
                    }
                }
            }
        },
        _ => {
            let output = Command::new(command).args(args).output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        print!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(_) => {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn handle_type(args: &Vec<&str>) {
    match BuiltInCommand::from_str(args[0]) {
        BuiltInCommand::Exit | BuiltInCommand::Echo | BuiltInCommand::Type => {
            println!("{} is a shell builtin", args[0])
        }
        _ => {
            let path = env::var("PATH").unwrap();
            let path_dirs = path.split(':').collect::<Vec<&str>>();

            for dir in path_dirs {
                let full_path = Path::new(dir).join(args[0]);
                if full_path.exists() {
                    println!("{} is {}", args[0], full_path.display());
                    return;
                }
            }

            println!("{} not found", args[0]);
        }
    }
}

fn handle_echo(args: &Vec<&str>) {
    println!("{}", args.join(" "));
}

fn handle_exit(args: &Vec<&str>) {
    if args.is_empty() {
        exit(0)
    }

    match args[0].trim().parse::<i32>() {
        Ok(val) => exit(val),
        Err(_) => exit(0),
    }
}
