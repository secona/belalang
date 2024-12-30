use std::error::Error;
use std::fs;
use std::path::PathBuf;

use belalang_compiler::compiler::CompilerBuilder;
use belalang_core::lexer::Lexer;
use belalang_core::parser::Parser;
use belalang_vm::vm::VMBuilder;

pub fn execute_file(filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::read(filename).expect("Unable to read file!");

    let lexer = Lexer::new(file.as_slice());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = CompilerBuilder::default().build();
    let mut vm = VMBuilder::default().build();

    let code = compiler.compile_program(program)?;
    vm.run(code)?;

    Ok(())
}
