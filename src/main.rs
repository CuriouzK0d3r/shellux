mod builtins;
mod interpreter;
mod lexer;
mod parser;

use anyhow::Result;
use clap::{Arg, Command};
use std::fs;
use std::io::{self, Write};

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
    println!();

    let mut input = String::new();
    let mut interpreter = Interpreter::new();

    loop {
        print!("shellux> ");
        io::stdout().flush()?;

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = input.trim();

                match line {
                    "exit" | "quit" => break,
                    "help" => {
                        println!("Available commands:");
                        println!("  exit, quit - Exit the REPL");
                        println!("  help - Show this help message");
                        println!("  tokens <code> - Show tokens for the given code");
                        println!("  ast <code> - Show AST for the given code");
                        println!("  Any other input will be executed as Shellux code");
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
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

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
