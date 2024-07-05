use std::error::Error;

use belalang_comp::compiler::Compiler;
use belalang_core::{lexer::Lexer, parser::Parser};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn compile(line: String) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(line.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::default();
    compiler.compile_program(program)?;
    println!("instructions: {:?}", compiler.instructions);
    println!("constants: {:?}", compiler.constants);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                if let Err(err) = compile(line) {
                    println!("ERROR: {}", err);
                }
            }
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("error reading line: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
