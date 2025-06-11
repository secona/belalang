use std::error::Error;
use std::fs;
use std::path::PathBuf;

use belalang_compiler::ast::Parser;
use belalang_compiler::codegen::Compiler;
use belalang_compiler::tokens::Lexer;
use belalang_vm::core::VM;

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
