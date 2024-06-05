use std::error::Error;
use std::fs;
use std::path::PathBuf;

use belalang::evaluator::builtins::Builtins;
use belalang::evaluator::Evaluator;
use belalang::lexer::Lexer;
use belalang::parser::Parser as BelalangParser;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

#[derive(clap::Parser)]
struct CLI {
    filename: Option<PathBuf>,
}

fn main() -> Result<(), ()> {
    let cli = CLI::parse();

    let _ = match cli.filename {
        Some(filename) => run_file(filename),
        None => repl(),
    };

    Ok(())
}

fn run_file(filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::read(filename).expect("Unable to read file!");

    let lexer = Lexer::new(file.into());
    let mut parser = BelalangParser::new(lexer);
    let builtins = Builtins::default();

    match parser.parse_program() {
        Ok(program) => {
            let mut ev = Evaluator::new(program, builtins);
            println!("{:?}", ev.evaluate());
        }
        Err(errors) => {
            println!("parser errors:");
            for error in errors {
                println!("- {}", error);
            }
        }
    }

    Ok(())
}

fn repl() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Belalang REPL v{}!\n", env!("CARGO_PKG_VERSION"));

    let mut rl = DefaultEditor::new()?;
    let mut ev = Evaluator::default();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let lexer = Lexer::new(line.as_bytes().into());
                let mut parser = BelalangParser::new(lexer);

                match parser.parse_program() {
                    Ok(program) => match ev.evaluate_statements(program.statements) {
                        Ok(evaluated) => println!("{}", evaluated),
                        Err(msg) => println!("{}", msg),
                    },
                    Err(errors) => {
                        println!("parser errors:");
                        for error in errors {
                            println!("- {}", error);
                        }
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {
                println!("\nSee you, space cowboy...");
                break;
            }
            Err(err) => {
                println!("Err: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
