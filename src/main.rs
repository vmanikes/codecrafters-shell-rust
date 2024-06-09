#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let stdin = io::stdin();
    let mut command = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut command).is_ok() {
        if command.contains("exit") {
            handle_exit(&command);
        } else if command.contains("echo") {
            handle_echo(&command);
        } else {
            println!("{}: command not found", command.trim());
            command.clear();
        }

        print!("$ ");
        io::stdout().flush().unwrap();
    }
}

fn handle_echo(command: &String) {
    let commands: Vec<&str> = command.split_whitespace().collect();
    let result: Vec<&str> = commands.into_iter().skip(1).collect();

    println!("{}", result.join(" ").trim());
}

fn handle_exit(command: &String) {
    let code: Vec<&str> = command.split_whitespace().collect();

    if code.len() == 1 {
        exit(0)
    }

    match code[1].trim().parse::<i32>() {
        Ok(val) => exit(val),
        _ => {exit(0)}
    }
}