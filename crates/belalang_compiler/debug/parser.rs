use std::error::Error;
use std::io::{self, Write};

use belalang_compiler::ast::Parser;
use belalang_compiler::tokens::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

        match parser.parse_program() {
            Ok(program) => println!("{:#?}", program.statements),
            Err(err) => println!("ERROR: {}", err),
        }
    }
}
