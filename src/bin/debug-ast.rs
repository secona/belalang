use std::io::{self, Write};

use belalang::{lexer::Lexer, parser::Parser};

fn main() {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        let lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

        match parser.parse_program() {
            Ok(program) => println!("{:#?}", program.statements),
            Err(err) => println!("{:?}", err),
        }
    }
}
