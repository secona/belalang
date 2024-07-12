#![cfg_attr(rustfmt, rustfmt_skip)]

use std::error::Error;

use belalang_comp::{code, compiler::{Code, Compiler}, object::Object};
use belalang_core::{lexer::Lexer, parser::Parser};

fn test_compile(input: &str) -> Result<Code, Box<dyn Error>> {
    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::default();
    let code = compiler.compile_program(program)?;

    Ok(code)
}

#[test]
fn integer_literals() {
    let code = test_compile("1; 2; 3;").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::POP,
        code::CONSTANT, 0, 1,
        code::POP,
        code::CONSTANT, 0, 2,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(1),
        Object::Integer(2),
        Object::Integer(3),
    ]);
}

#[test]
fn booleans() {
    let code = test_compile("true; false;").unwrap();

    assert_eq!(code.instructions, vec![
        code::TRUE,
        code::POP,
        code::FALSE,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![]);
}

fn test_compile_infix(op: &str, code: u8) {
    let input = format!("1 {} 3;", op);
    let compiled = test_compile(&input).unwrap();

    assert_eq!(compiled.instructions, vec![
        code::CONSTANT, 0, 0,
        code::CONSTANT, 0, 1,
        code,
        code::POP,
    ]);

    assert_eq!(compiled.constants, vec![
        Object::Integer(1),
        Object::Integer(3),
    ]);
}

#[test]
fn infix_expressions() {
    test_compile_infix("+", code::ADD);
    test_compile_infix("-", code::SUB);
    test_compile_infix("*", code::MUL);
    test_compile_infix("/", code::DIV);
    test_compile_infix("%", code::MOD);
    test_compile_infix("==", code::EQ);
    test_compile_infix("!=", code::NE);
    test_compile_infix("<", code::LT);
    test_compile_infix("<=", code::LE);
    test_compile_infix(">", code::GT);
    test_compile_infix(">=", code::GE);
}

#[test]
fn prefix_expressions() {
    let code = test_compile("-5;").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::MINUS,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(5),
    ]);
}

#[test]
fn if_expressions() {
    let code = test_compile("if (true) { 10 }; 9;").unwrap();

    assert_eq!(code.instructions, vec![
        code::TRUE,
        code::JUMP_IF_FALSE, 0, 7,
        code::CONSTANT, 0, 0,
        code::POP,
        code::CONSTANT, 0, 1,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(10),
        Object::Integer(9),
    ]);
}
