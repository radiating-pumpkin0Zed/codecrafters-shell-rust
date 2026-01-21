#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    print!("$ ");
    io::stdout().flush().unwrap();

    // TODO: Read user input and execute commands
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}: command not found", input.trim());

    // TODO: Implement the REPL(Read-Eval-Print Loop)
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        if command == "exit" {
            break;
        }

        // Here you would normally parse and execute the command
        println!("{}: command not found", command);
    }
}
