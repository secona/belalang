use std::error::Error;

use belalang_compiler::{lexer::Lexer, token::Token};
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let mut lexer = Lexer::new(line.as_bytes());

                loop {
                    match lexer.next_token() {
                        Ok(Token::EOF) => break,
                        Ok(token) => println!("{:?}", token),
                        Err(err) => println!("ERROR: {}", err),
                    };
                }
            }
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("error reading line: {:?}", err);
                break;
            }
        };
    }

    Ok(())
}
