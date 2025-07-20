use std::error::Error;
use std::fs;
use std::path::PathBuf;

use belc::ast::Parser;
use belc::codegen::Compiler;
use belc::tokens::Lexer;
use belvm::core::VM;

pub fn run(filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(filename).expect("Unable to read file!");

    let lexer = Lexer::new(&file);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::default();
    let mut vm = VM::default();

    let code = compiler.compile_program(program)?;
    vm.run(code)?;

    Ok(())
}
