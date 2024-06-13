use std::{error::Error, fs, path::PathBuf};

use belalang::{evaluator::Evaluator, lexer::Lexer, parser::Parser};
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn run_file(filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::read(filename).expect("Unable to read file!");

    let lexer = Lexer::new(file.as_slice());
    let mut parser = Parser::new(lexer);
    let mut ev = Evaluator::default();

    let program = match parser.parse_program() {
        Ok(program) => Ok(program),
        Err(err) => Err(err
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n ")),
    }?;

    ev.eval_program(program)?;
    Ok(())
}

pub fn repl() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Belalang REPL v{}!\n", env!("CARGO_PKG_VERSION"));

    let mut rl = DefaultEditor::new()?;
    let mut ev = Evaluator::default();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let lexer = Lexer::new(line.as_bytes().into());
                let mut parser = Parser::new(lexer);

                match parser.parse_program() {
                    Ok(program) => match ev.eval_program(program) {
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
