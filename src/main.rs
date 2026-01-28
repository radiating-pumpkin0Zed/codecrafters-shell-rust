use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command as ProcessCommand;

const BUILT_IN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];
enum Command {
    ExitCommand,
    EchoCommand { 
        display_string: String,
    },
    TypeCommand { 
        command_name: String, 
    },
    ExternalCommand {
        command_name: String,
        args: Vec<String>,
    },
    CommandNotFound,
}

impl Command {
    fn from_input(input: &str) -> Self {
        let input = input.trim();
        if input == "exit" {
            return Self::ExitCommand;
        };
        if let Some(pos) = input.find("echo ") {
            if pos == 0 {
                return Self::EchoCommand {
                    display_string: input["echo ".len()..].to_string(),
                };
            }
        }
        if let Some(pos) = input.find("type ") {
            if pos == 0 {
                return Self::TypeCommand {
                    command_name: input["type ".len()..].to_string(),
                };
            }
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Self::CommandNotFound;
        }

        let command_name = parts[0].to_string();
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        Self::ExternalCommand { command_name, args }
    }
}

fn find_executable(command_name: &str) -> Option<String> {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let full_path = Path::new(dir).join(command_name);
            if full_path.exists() {
                if let Ok(metadata) = fs::metadata(&full_path) {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        return Some(full_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = Command::from_input(&input);

        match command {
            Command::ExitCommand => break,
            Command::EchoCommand { display_string } => println!("{}", display_string),
            Command::TypeCommand { command_name } => {
                if BUILT_IN_COMMANDS.contains(&command_name.as_str()) {
                    println!("{} is a shell builtin", command_name);
                } else if let Some(full_path) = find_executable(&command_name) {
                    println!("{} is {}", command_name, full_path);
                } else {
                    println!("{}: not found", command_name)
                }
            }
            Command::ExternalCommand { command_name, args } => {
                if let Some(full_path) = find_executable(&command_name) {
                    let mut cmd = ProcessCommand::new(full_path);
                    cmd.arg0(&command_name);
                    for arg in &args {
                        cmd.arg(arg);
                    }

                    match cmd.status() {
                        Ok(_) => {}
                        Err(_) => {
                            println!("{}: command not found", input.trim());
                        }
                    }
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
            Command::CommandNotFound => println!("{}: command not found", input.trim()),
        }
    }

    
}