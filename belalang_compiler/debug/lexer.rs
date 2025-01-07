use std::error::Error;
use std::io::{self, Write};

use belalang_compiler::tokens::Lexer;
use belalang_compiler::tokens::Token;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input.as_bytes());

        loop {
            match lexer.next_token() {
                Ok(Token::EOF) => break,
                Ok(token) => println!("{:?}", token),
                Err(err) => println!("ERROR: {}", err),
            };
        }
    }
}
