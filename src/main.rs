use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command as ProcessCommand;

const BUILT_IN_COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];
enum Command {
    ExitCommand,
    EchoCommand { 
        display_string: String,
    },
    TypeCommand { 
        command_name: String, 
    },
    PwdCommand,
    CdCommand {
        target: Option<String>,
    },
    ExternalCommand {
        command_name: String,
        args: Vec<String>,
    },
    CommandNotFound,
}

fn split_with_single_quotes(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;

    for ch in input.chars() {
        if ch == '\'' {
            in_single_quotes = !in_single_quotes;
            continue;
        }

        if ch.is_whitespace() && !in_single_quotes {
            if !current.is_empty() {
                parts.push(std::mem::take(&mut current));
            }
            continue;
        }

        current.push(ch);
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

impl Command {
    fn from_input(input: &str) -> Self {
        let input = input.trim();
        let parts = split_with_single_quotes(input);

        if parts.is_empty() {
            return Self::CommandNotFound;
        }

        if parts[0] == "echo" {
            return Self::EchoCommand {
                display_string: parts[1..].join(" "),
            };
        }

        if input == "exit" {
            return Self::ExitCommand;
        };
        if parts[0] == "type" {
            if parts.len() > 1 {
                return Self::TypeCommand {
                    command_name: parts[1].clone(),
                };
            }
            return Self::CommandNotFound;
        }
        if parts[0] == "pwd" {
            return Self::PwdCommand;
        }
        if parts[0] == "cd" && parts.len() == 1 {
            return Self::CdCommand { target: None };
        }
        if parts[0] == "cd" {
            return Self::CdCommand {
                target: Some(parts[1].clone()),
            };
        }

        let command_name = parts[0].clone();
        let args: Vec<String> = parts[1..].to_vec();

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
            Command::PwdCommand => match env::current_dir() {
                Ok(dir) => println!("{}", dir.display()),
                Err(_) => println!("pwd: error retrieving current directory"),
            },
            Command::CdCommand { target } => {
                let resolved = match target {
                    None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
                    Some(path) if path == "~" => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
                    Some(path) => path,
                };
                if let Err(err) = env::set_current_dir(&resolved) {
                    let msg = match err.kind() {
                        ErrorKind::NotFound => "No such file or directory",
                        _ => "Error",
                    };
                    println!("cd: {}: {}", resolved, msg);
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
