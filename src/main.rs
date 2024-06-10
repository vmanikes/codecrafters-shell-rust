use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    let builtin_commands: Vec<&str> = vec!["exit", "echo", "type"];

    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        let command = input.split_whitespace().collect::<Vec<&str>>();
        let parameters = command.clone().into_iter().skip(1).collect::<Vec<&str>>();

        if command.len() >= 1 {
            match command[0] {
                "exit" => {
                    handle_exit(&parameters);
                },
                "echo" => {
                    handle_echo(&parameters);
                },
                "type" => {
                    handle_type(&builtin_commands, &parameters);
                },
                _ => {
                    execute_command(command[0], &parameters);
                    // println!("{}: command not found", command[0].trim());
                }
            }
        }

        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}

fn execute_command(command: &str, parameters: &Vec<&str>) {
    let output = Command::new(command).args(parameters).output();

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

fn handle_type(builtin_commands: &Vec<&str>, parameters: &Vec<&str>) {
    if builtin_commands.contains(&parameters[0]) {
        println!("{} is a shell builtin", parameters[0])
    } else {
        let path = env::var("PATH").unwrap();
        let path_dirs = path.split(':').collect::<Vec<&str>>();

        for dir in path_dirs {
            let full_path = Path::new(dir).join(parameters[0]);
            if full_path.exists() {
                println!("{} is {}", parameters[0], full_path.display());
                return;
            }
        }

        println!("{} not found", parameters[0]);
    }
}

fn handle_echo(parameters: &Vec<&str>) {
    println!("{}", parameters.join(" "));
}

fn handle_exit(parameters: &Vec<&str>) {
    if parameters.is_empty() {
        exit(0)
    }

    match parameters[0].trim().parse::<i32>() {
        Ok(val) => exit(val),
        Err(_) => exit(0),
    }
}
