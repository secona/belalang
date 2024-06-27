use std::{
    error::Error,
    io::{self, Write},
};

use belalang_core::{lexer::Lexer, token::Token};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        let mut lexer = Lexer::new(input.as_bytes());

        loop {
            match lexer.next_token() {
                Ok(Token::EOF) => break,
                Ok(token) => println!("{:?}", token),
                Err(err) => println!("ERROR: {:?}", err)
            };
        }
    }
}
