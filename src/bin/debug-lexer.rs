use std::io::{self, Write};

use belalang::{lexer::Lexer, token::Token};

fn main() {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        let mut lexer = Lexer::new(input.as_bytes());

        loop {
            let token = lexer.next_token();
            println!("{:?}", token);

            if let Token::EOF = token {
                break;
            }
        }
    }
}
