#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    let mut input = String::new();
  
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
        
        //Echo command
        let parts: Vec<&str> = command.split_whitespace().collect();

        if !parts.is_empty() && parts[0] == "echo" {
            println!("{}", parts[1..].join(" "));
            continue;
        }

    println!("{}: command not found", command);
    }
}