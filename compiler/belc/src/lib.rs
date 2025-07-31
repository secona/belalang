#![feature(assert_matches)]

use belc_ast::Parser;
use belc_lexer::Lexer;
use belvm_bytecode::Bytecode;

use crate::codegen::Compiler;
pub use crate::codegen::disassembler::disassemble;

pub mod codegen;
pub mod error;

pub fn compile(source: &str) -> Bytecode {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();

    let mut compiler = Compiler::default();
    compiler.compile_program(program).unwrap()
}
