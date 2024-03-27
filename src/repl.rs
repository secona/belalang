use std::io::{self, Write};

use crate::{lexer::Lexer, token::Token};

pub struct Repl {}

impl Repl {
    pub fn start() {
        loop {
            print!(">> ");
            let _ = io::stdout().flush();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading from STDIN");

            let mut lexer = Lexer::new(input.into_bytes().into_boxed_slice());

            loop {
                let tok = lexer.next_token();

                println!("{:?}", tok);

                if matches!(tok, Token::EOF) {
                    break;
                }
            }
        }
    }
}
