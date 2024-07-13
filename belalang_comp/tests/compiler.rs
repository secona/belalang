#![cfg_attr(rustfmt, rustfmt_skip)]

use std::error::Error;

use belalang_comp::object::{Object, Function};
use belalang_comp::compiler::{Code, Compiler};
use belalang_comp::code;
use belalang_core::parser::Parser;
use belalang_core::lexer::Lexer;

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

fn test_compile_infix(op: &str, code: u8, reversed: bool) {
    let input = format!("1 {} 3;", op);
    let compiled = test_compile(&input).unwrap();

    assert_eq!(compiled.instructions, vec![
        code::CONSTANT, 0, 0,
        code::CONSTANT, 0, 1,
        code,
        code::POP,
    ]);

    assert_eq!(compiled.constants, if reversed {
        vec![
            Object::Integer(3),
            Object::Integer(1),
        ]
    } else {
        vec![
            Object::Integer(1),
            Object::Integer(3),
        ]
    });
}

#[test]
fn infix_expressions() {
    test_compile_infix("+", code::ADD, false);
    test_compile_infix("-", code::SUB, false);
    test_compile_infix("*", code::MUL, false);
    test_compile_infix("/", code::DIV, false);
    test_compile_infix("%", code::MOD, false);
    test_compile_infix("==", code::EQUAL, false);
    test_compile_infix("!=", code::NOT_EQUAL, false);
    test_compile_infix("<", code::LESS_THAN, false);
    test_compile_infix("<=", code::LESS_THAN_EQUAL, false);
    test_compile_infix(">", code::LESS_THAN, true);
    test_compile_infix(">=", code::LESS_THAN_EQUAL, true);
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
    let code = test_compile("if (1 == 1) { 10 }; 9;").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::CONSTANT, 0, 1,
        code::EQUAL,
        code::JUMP_IF_FALSE, 0, 6,
        code::CONSTANT, 0, 2,
        code::JUMP, 0, 1,
        code::NULL,
        code::POP,
        code::CONSTANT, 0, 3,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(1),
        Object::Integer(1),
        Object::Integer(10),
        Object::Integer(9),
    ]);
}

#[test]
fn if_else_expressions() {
    let code = test_compile("if (true) { 10 } else { 11 };").unwrap();

    assert_eq!(code.instructions, vec![
        code::TRUE,
        code::JUMP_IF_FALSE, 0, 6,
        code::CONSTANT, 0, 0,
        code::JUMP, 0, 3,
        code::CONSTANT, 0, 1,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(10),
        Object::Integer(11),
    ]);
}

#[test]
fn if_else_if_expressions() {
    let code = test_compile("
        if (true) { 10 }
        else if (true) { 11 }
        else { 12 };
    ").unwrap();

    assert_eq!(code.instructions, vec![
        code::TRUE,
        code::JUMP_IF_FALSE, 0, 6,
        code::CONSTANT, 0, 0,
        code::JUMP, 0, 13,
        code::TRUE,
        code::JUMP_IF_FALSE, 0, 6,
        code::CONSTANT, 0, 1,
        code::JUMP, 0, 3,
        code::CONSTANT, 0, 2,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(10),
        Object::Integer(11),
        Object::Integer(12),
    ]);
}

#[test]
fn var() {
    let code = test_compile("x := 12; x = 11; x;").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::SET_GLOBAL, 0, 0,
        code::POP,
        code::CONSTANT, 0, 1,
        code::SET_GLOBAL, 0, 0,
        code::POP,
        code::GET_GLOBAL, 0, 0,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(12),
        Object::Integer(11),
    ]);
}

#[test]
fn var_assignment_ops() {
    let code = test_compile("x := 1; x += 1;").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::SET_GLOBAL, 0, 0,
        code::POP,
        code::GET_GLOBAL, 0, 0,
        code::CONSTANT, 0, 1,
        code::ADD,
        code::SET_GLOBAL, 0, 0,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(1),
        Object::Integer(1),
    ]);
}

#[test]
fn block_expression() {
    let code = test_compile("{ x := 12; };").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::SET_LOCAL, 0,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(12),
    ]);
}

#[test]
fn function_expressions() {
    let code = test_compile("ten := fn() { 10 };").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 1,
        code::SET_GLOBAL, 0, 0,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(10),
        Object::Function(Function {
            instructions: vec![
                code::CONSTANT, 0, 0,
                code::RETURN_VALUE,
            ],
            arity: 0
        })
    ]);
}

#[test]
fn function_with_args_expressions() {
    let code = test_compile("add := fn(a, b) { a + b }; three := add(1, 2);").unwrap();

    assert_eq!(code.instructions, vec![
        code::CONSTANT, 0, 0,
        code::SET_GLOBAL, 0, 0,
        code::POP,
        code::CONSTANT, 0, 1,
        code::CONSTANT, 0, 2,
        code::GET_GLOBAL, 0, 0,
        code::CALL,
        code::SET_GLOBAL, 0, 1,
        code::POP,
    ]);

    assert_eq!(code.constants, vec![
        Object::Function(Function {
            instructions: vec![
                code::GET_LOCAL, 0,
                code::GET_LOCAL, 1,
                code::ADD,
                code::RETURN_VALUE,
            ],
            arity: 2,
        }),
        Object::Integer(2),
        Object::Integer(1),
    ]);
}
