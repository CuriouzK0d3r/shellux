mod builtins;
mod interpreter;
mod lexer;
mod parser;

use anyhow::Result;
use clap::{Arg, Command};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() -> Result<()> {
    let matches = Command::new("shellux")
        .version("0.1.0")
        .about("A modern scripting language to replace bash scripting")
        .arg(
            Arg::new("file")
                .help("The script file to execute")
                .value_name("FILE")
                .index(1),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Run in interactive mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("tokens")
                .long("tokens")
                .help("Show tokenized output for debugging")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ast")
                .long("ast")
                .help("Show AST output for debugging")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("interactive") {
        run_interactive_mode()
    } else if let Some(file) = matches.get_one::<String>("file") {
        run_file(file, matches.get_flag("tokens"), matches.get_flag("ast"))
    } else {
        eprintln!("Usage: shellux [file] or shellux -i");
        std::process::exit(1);
    }
}

fn run_file(filename: &str, show_tokens: bool, show_ast: bool) -> Result<()> {
    let source = fs::read_to_string(filename)?;

    if show_tokens {
        tokenize_and_display(&source)?
    } else if show_ast {
        parse_and_display(&source)?
    } else {
        execute_source(&source)?
    }

    Ok(())
}

fn run_interactive_mode() -> Result<()> {
    println!("Shellux v0.1.0 - Interactive Mode");
    println!("Type 'exit' to quit, 'help' for help");
    println!("Use arrow keys to navigate command history");
    println!();

    let mut rl =
        DefaultEditor::new().map_err(|e| anyhow::anyhow!("Failed to create editor: {}", e))?;
    let mut interpreter = Interpreter::new();

    // Try to load history file
    let history_file = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(|home| format!("{}/.shellux_history", home))
        .unwrap_or_else(|_| ".shellux_history".to_string());

    let _ = rl.load_history(&history_file);

    loop {
        match rl.readline("shellux> ") {
            Ok(line) => {
                let line = line.trim();

                // Add non-empty commands to history
                if !line.is_empty() {
                    rl.add_history_entry(line)
                        .map_err(|e| anyhow::anyhow!("Failed to add history: {}", e))?;
                }

                match line {
                    "exit" | "quit" => break,
                    "help" => {
                        println!("Available commands:");
                        println!("  exit, quit - Exit the REPL");
                        println!("  help - Show this help message");
                        println!("  tokens <code> - Show tokens for the given code");
                        println!("  ast <code> - Show AST for the given code");
                        println!("  Any other input will be executed as Shellux code");
                        println!();
                        println!("External Commands:");
                        println!("  ls, pwd, date, etc. - Run system commands directly");
                        println!("  echo(\"message\") - Run commands with arguments");
                        println!("  $(command) - Command substitution (capture output)");
                        println!("  run(\"cmd\", \"args\") - Run complex commands");
                        println!();
                        println!("Built-in Commands:");
                        println!("  echo, print, cd, pwd - Shell built-ins");
                        println!("  read_file, write_file - File operations");
                        println!("  input, len, to_string - Utility functions");
                        println!();
                        println!("Navigation & Editing:");
                        println!("  ↑/↓ (Up/Down) - Navigate command history");
                        println!("  ←/→ (Left/Right) - Move cursor within current line");
                        println!("  Ctrl+A - Move to beginning of line");
                        println!("  Ctrl+E - Move to end of line");
                        println!("  Ctrl+L - Clear screen");
                        println!("  Ctrl+C - Cancel current input (continue session)");
                        println!("  Ctrl+D - Exit interactive mode (EOF)");
                        println!("  Home/End - Move to start/end of line");
                        println!("  Ctrl+W - Delete word backwards");
                        println!("  Ctrl+K - Delete from cursor to end of line");
                        println!("  Ctrl+U - Delete entire line");
                        println!();
                        println!("History:");
                        println!("  Command history persists between sessions");
                        println!("  History file: ~/.shellux_history");
                    }
                    line if line.starts_with("tokens ") => {
                        let code = &line[7..]; // Skip "tokens "
                        if let Err(e) = tokenize_and_display(code) {
                            eprintln!("Error: {}", e);
                        }
                    }
                    line if line.starts_with("ast ") => {
                        let code = &line[4..]; // Skip "ast "
                        if let Err(e) = parse_and_display(code) {
                            eprintln!("Error: {}", e);
                        }
                    }
                    "" => continue,
                    code => {
                        if let Err(e) = execute_source_repl(code, &mut interpreter) {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }

    // Save history before exiting
    let _ = rl.save_history(&history_file);
    println!("Goodbye!");
    Ok(())
}

fn tokenize_and_display(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!(
            "  {}: {:?} (line {}, column {})",
            i, token.token_type, token.line, token.column
        );
    }

    Ok(())
}

fn parse_and_display(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    println!("AST:");
    println!("{:#?}", program);

    Ok(())
}

fn execute_source(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // Check for lexer errors
    for token in &tokens {
        if let lexer::token::TokenType::Error(msg) = &token.token_type {
            eprintln!(
                "Lexer error at line {}, column {}: {}",
                token.line, token.column, msg
            );
            return Ok(());
        }
    }

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return Ok(());
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(program) {
        Ok(_) => {} // Successful execution
        Err(e) => {
            eprintln!("Runtime error: {}", e);
        }
    }

    Ok(())
}

fn execute_source_repl(source: &str, interpreter: &mut Interpreter) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // Check for lexer errors
    for token in &tokens {
        if let lexer::token::TokenType::Error(msg) = &token.token_type {
            eprintln!(
                "Lexer error at line {}, column {}: {}",
                token.line, token.column, msg
            );
            return Ok(());
        }
    }

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return Ok(());
        }
    };

    match interpreter.interpret(program) {
        Ok(value) => {
            // In REPL mode, show the result if it's not nil
            match value {
                interpreter::Value::Nil => {} // Don't show nil values
                _ => println!("=> {}", value.to_string()),
            }
        }
        Err(e) => {
            eprintln!("Runtime error: {}", e);
        }
    }

    Ok(())
}
