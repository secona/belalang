use std::error::Error;

use belalang_comp::compiler::Compiler;
use belalang_core::{lexer::Lexer, parser::Parser};
use belalang_vm::vm::VM;
use rustyline::{error::ReadlineError, DefaultEditor};

fn run(line: String) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(line.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::default();
    compiler.compile_program(program)?;

    let mut vm = VM::new(compiler);
    vm.run()?;

    println!("stack: {:?}", vm.stack);
    println!("sp: {}", vm.sp);
    println!("last popped: {:?}", vm.last_popped);

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
