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
            let token = lexer.next_token()?;
            println!("{:?}", token);

            if let Token::EOF = token {
                break;
            }
        }
    }
}