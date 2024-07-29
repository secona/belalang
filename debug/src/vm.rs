use std::error::Error;

use belalang_compiler::compiler::CompilerBuilder;
use belalang_core::{lexer::Lexer, parser::Parser};
use belalang_vm::vm::VMBuilder;
use rustyline::{error::ReadlineError, DefaultEditor};

fn run(line: String) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(line.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = CompilerBuilder::default().build();
    let code = compiler.compile_program(program)?;

    let mut vm = VMBuilder::default().build();
    vm.run(code)?;

    println!("stack: {:#?}", vm.stack);
    println!("globals: {:#?}", vm.globals);
    println!("frame: {:#?}", vm.frame);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                if let Err(err) = run(line) {
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
