use std::error::Error;

use belalang_core::{lexer::Lexer, parser::Parser};
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let lexer = Lexer::new(line.as_bytes());
                let mut parser = Parser::new(lexer);

                match parser.parse_program() {
                    Ok(program) => println!("{:#?}", program.statements),
                    Err(err) => println!("ERROR: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("error reading line: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
