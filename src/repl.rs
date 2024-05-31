use std::io::{self, Write};

use crate::{evaluator, lexer::Lexer, parser};

pub struct Repl {}

impl Repl {
    pub fn start() {
        loop {
            print!(">>> ");
            let _ = io::stdout().flush();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading from STDIN");

            let lexer = Lexer::new(input.into_bytes().into_boxed_slice());
            let mut parser = parser::Parser::new(lexer);

            match parser.parse_program() {
                Ok(program) => {
                    println!("{}", evaluator::eval_program(program));
                }
                Err(errors) => {
                    println!("parser errors:");
                    for error in errors {
                        println!("- {}", error);
                    }
                }
            };
        }
    }
}
