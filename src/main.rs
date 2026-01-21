#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    print!("$ ");
    io::stdout().flush().unwrap();

    // TODO: Read user input and execute commands
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}: command not found", input.trim());
}
