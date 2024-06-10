use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

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
                    println!("{}: command not found", command[0].trim());
                }
            }
        }

        input.clear();

        print!("$ ");
        io::stdout().flush().unwrap();
    }
}

fn handle_type(builtin_commands: &Vec<&str>, parameters: &Vec<&str>) {
    if builtin_commands.contains(&parameters[0]) {
        println!("{} is a shell builtin", parameters[0])
    } else {
        let path = env::var("PATH").unwrap();

        let path_dirs = path.split(":").collect::<Vec<&str>>();

        for dir in path_dirs {
            if Path::new(dir).join(parameters[0]).exists() {
                println!("{} in {}", parameters[0], dir);
                return;
            }
        }

        println!("{} not found", parameters[0])
    }
}

fn handle_echo(parameters: &Vec<&str>) {
    println!("{}", parameters.join(" ").trim());
}

fn handle_exit(parameters: &Vec<&str>) {
    if parameters.len() == 0 {
        exit(0)
    }

    match parameters[0].trim().parse::<i32>() {
        Ok(val) => exit(val),
        _ => {exit(0)}
    }
}