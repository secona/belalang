use std::error::Error;
use std::fs;
use std::path::PathBuf;

use belalang_compiler::codegen::Compiler;
use belalang_compiler::tokens::Lexer;
use belalang_compiler::ast::Parser;
use belalang_vm::vm::VM;

pub fn execute_file(filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::read(filename).expect("Unable to read file!");

    let lexer = Lexer::new(file.as_slice());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::default();
    let mut vm = VM::default();

    let code = compiler.compile_program(program)?;
    vm.run(code)?;

    Ok(())
}
