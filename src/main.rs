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
        }

        println!("{}: command not found", command.trim());

        command.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}

fn handle_exit(command: &String) {
    let code: Vec<&str> = command.split(" ").collect();

    exit(code[1].trim().parse::<i32>().unwrap());
}