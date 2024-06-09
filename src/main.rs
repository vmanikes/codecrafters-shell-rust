#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut command = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut command).is_ok() {
        println!("{}: command not found", command.trim());

        command.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
