use std::io;
use std::io::Write;

use belalang_core::{lexer::Lexer, parser::Parser};
use belalang_comp::compiler::Compiler;

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
            Ok(program) => {
                let mut compiler = Compiler::default();
                match compiler.compile_program(program) {
                    Ok(_) => {
                        println!("instructions: {:?}", compiler.instructions);
                        println!("constants: {:?}", compiler.constants);
                    },
                    Err(err) => println!("ERROR: {:?}", err),
                };
            },
            Err(err) => println!("ERROR: {:?}", err),
        }
    }
}
