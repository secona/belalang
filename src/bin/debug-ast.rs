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

        let lexer = Lexer::new(input.into_bytes().into_boxed_slice());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parsing errors");

        println!("{:#?}", program.statements);
    }
}
