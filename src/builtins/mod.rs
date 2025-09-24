use crate::interpreter::{Environment, Value};
use anyhow::{anyhow, Result};
use std::fs;
use std::io::{self, Write};

pub fn register_builtins(env: &mut Environment) {
    // IO functions
    env.define(
        "print".to_string(),
        Value::Function {
            name: "print".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "println".to_string(),
        Value::Function {
            name: "println".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "input".to_string(),
        Value::Function {
            name: "input".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    // File operations
    env.define(
        "read_file".to_string(),
        Value::Function {
            name: "read_file".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "write_file".to_string(),
        Value::Function {
            name: "write_file".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    // String functions
    env.define(
        "len".to_string(),
        Value::Function {
            name: "len".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "to_string".to_string(),
        Value::Function {
            name: "to_string".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "to_int".to_string(),
        Value::Function {
            name: "to_int".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "to_float".to_string(),
        Value::Function {
            name: "to_float".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    // System functions
    env.define(
        "exit".to_string(),
        Value::Function {
            name: "exit".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    // Shell built-ins
    env.define(
        "echo".to_string(),
        Value::Function {
            name: "echo".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "cd".to_string(),
        Value::Function {
            name: "cd".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "pwd".to_string(),
        Value::Function {
            name: "pwd".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );

    env.define(
        "run".to_string(),
        Value::Function {
            name: "run".to_string(),
            parameters: vec![],
            body: vec![],
            closure: Environment::new(),
        },
    );
}

pub fn call_builtin(name: &str, args: &[Value]) -> Result<Value> {
    match name {
        "print" => {
            if args.is_empty() {
                println!();
            } else {
                let output = args
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{}", output);
            }
            Ok(Value::Nil)
        }

        "println" => {
            if args.is_empty() {
                println!();
            } else {
                let output = args
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{}", output);
            }
            Ok(Value::Nil)
        }

        "input" => {
            if !args.is_empty() {
                let prompt = args[0].to_string();
                print!("{}", prompt);
                io::stdout().flush().unwrap();
            }

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| anyhow!("Failed to read input: {}", e))?;

            // Remove trailing newline
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }

            Ok(Value::String(input))
        }

        "read_file" => {
            if args.len() != 1 {
                return Err(anyhow!("read_file expects 1 argument, got {}", args.len()));
            }

            let filename = args[0].to_string();
            let content = fs::read_to_string(&filename)
                .map_err(|e| anyhow!("Failed to read file {}: {}", filename, e))?;

            Ok(Value::String(content))
        }

        "write_file" => {
            if args.len() != 2 {
                return Err(anyhow!(
                    "write_file expects 2 arguments, got {}",
                    args.len()
                ));
            }

            let filename = args[0].to_string();
            let content = args[1].to_string();

            fs::write(&filename, content)
                .map_err(|e| anyhow!("Failed to write file {}: {}", filename, e))?;

            Ok(Value::Nil)
        }

        "len" => {
            if args.len() != 1 {
                return Err(anyhow!("len expects 1 argument, got {}", args.len()));
            }

            let length = match &args[0] {
                Value::String(s) => s.len() as i64,
                Value::Array(arr) => arr.len() as i64,
                Value::Map(map) => map.len() as i64,
                _ => {
                    return Err(anyhow!(
                        "len not supported for type {}",
                        args[0].type_name()
                    ))
                }
            };

            Ok(Value::Integer(length))
        }

        "to_string" => {
            if args.len() != 1 {
                return Err(anyhow!("to_string expects 1 argument, got {}", args.len()));
            }

            Ok(Value::String(args[0].to_string()))
        }

        "to_int" => {
            if args.len() != 1 {
                return Err(anyhow!("to_int expects 1 argument, got {}", args.len()));
            }

            match &args[0] {
                Value::Integer(i) => Ok(Value::Integer(*i)),
                Value::Float(f) => Ok(Value::Integer(*f as i64)),
                Value::String(s) => {
                    let parsed = s
                        .parse::<i64>()
                        .map_err(|e| anyhow!("Cannot convert '{}' to integer: {}", s, e))?;
                    Ok(Value::Integer(parsed))
                }
                Value::Boolean(true) => Ok(Value::Integer(1)),
                Value::Boolean(false) => Ok(Value::Integer(0)),
                _ => Err(anyhow!("Cannot convert {} to integer", args[0].type_name())),
            }
        }

        "to_float" => {
            if args.len() != 1 {
                return Err(anyhow!("to_float expects 1 argument, got {}", args.len()));
            }

            match &args[0] {
                Value::Float(f) => Ok(Value::Float(*f)),
                Value::Integer(i) => Ok(Value::Float(*i as f64)),
                Value::String(s) => {
                    let parsed = s
                        .parse::<f64>()
                        .map_err(|e| anyhow!("Cannot convert '{}' to float: {}", s, e))?;
                    Ok(Value::Float(parsed))
                }
                Value::Boolean(true) => Ok(Value::Float(1.0)),
                Value::Boolean(false) => Ok(Value::Float(0.0)),
                _ => Err(anyhow!("Cannot convert {} to float", args[0].type_name())),
            }
        }

        "exit" => {
            let code = if args.is_empty() {
                0
            } else {
                match &args[0] {
                    Value::Integer(i) => *i as i32,
                    _ => return Err(anyhow!("exit expects integer argument")),
                }
            };

            std::process::exit(code);
        }

        "echo" => {
            let output = args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", output);
            Ok(Value::Nil)
        }

        "cd" => {
            let path = if args.is_empty() {
                std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
            } else {
                args[0].to_string()
            };

            std::env::set_current_dir(&path)
                .map_err(|e| anyhow!("Failed to change directory to '{}': {}", path, e))?;

            Ok(Value::Nil)
        }

        "pwd" => {
            let current_dir = std::env::current_dir()
                .map_err(|e| anyhow!("Failed to get current directory: {}", e))?;

            let path_str = current_dir.to_string_lossy().to_string();
            println!("{}", path_str);
            Ok(Value::Nil)
        }

        "run" => {
            if args.is_empty() {
                return Err(anyhow!("run expects at least 1 argument"));
            }

            let command = args[0].to_string();
            let mut cmd_args = Vec::new();

            // If there's a second argument, split it by spaces to get shell-style args
            if args.len() > 1 {
                let args_str = args[1].to_string();
                for arg in args_str.split_whitespace() {
                    cmd_args.push(arg.to_string());
                }
            }

            // Execute the command
            let mut cmd = std::process::Command::new(&command);
            cmd.args(&cmd_args);

            match cmd.output() {
                Ok(output) => {
                    // Print stderr if there's any error output
                    if !output.stderr.is_empty() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprint!("{}", stderr);
                    }

                    // Print stdout
                    if !output.stdout.is_empty() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        print!("{}", stdout);
                    }

                    if output.status.success() {
                        Ok(Value::Nil)
                    } else {
                        let exit_code = output.status.code().unwrap_or(-1);
                        Err(anyhow!(
                            "Command '{}' failed with exit code {}",
                            command,
                            exit_code
                        ))
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        Err(anyhow!("Command not found: {}", command))
                    } else {
                        Err(anyhow!("Failed to execute command '{}': {}", command, e))
                    }
                }
            }
        }

        _ => Err(anyhow!("Unknown built-in function: {}", name)),
    }
}

pub fn is_builtin(name: &str) -> bool {
    matches!(
        name,
        "print"
            | "println"
            | "input"
            | "read_file"
            | "write_file"
            | "len"
            | "to_string"
            | "to_int"
            | "to_float"
            | "exit"
            | "echo"
            | "cd"
            | "pwd"
            | "run"
    )
}
