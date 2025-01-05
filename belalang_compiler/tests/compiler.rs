#![cfg_attr(rustfmt, rustfmt_skip)]

use std::error::Error;

use belalang_compiler::codegen::Compiler;
use belalang_compiler::ast::Parser;
use belalang_compiler::tokens::Lexer;
use belalang_vm::object::Object;
use belalang_vm::opcode;
use belalang_vm::bytecode::Bytecode;

fn test_compile(input: &str) -> Result<Bytecode, Box<dyn Error>> {
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
        opcode::CONSTANT, 0, 0,
        opcode::POP,
        opcode::CONSTANT, 0, 1,
        opcode::POP,
        opcode::CONSTANT, 0, 2,
        opcode::POP,
        opcode::RETURN_VALUE,
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
        opcode::TRUE,
        opcode::POP,
        opcode::FALSE,
        opcode::POP,
        opcode::RETURN_VALUE,
    ]);

    assert_eq!(code.constants, vec![]);
}

fn test_compile_infix(op: &str, code: u8, reversed: bool) {
    let input = format!("1 {} 3;", op);
    let compiled = test_compile(&input).unwrap();

    assert_eq!(compiled.instructions, vec![
        opcode::CONSTANT, 0, 0,
        opcode::CONSTANT, 0, 1,
        code,
        opcode::POP,
        opcode::RETURN_VALUE,
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
    test_compile_infix("+", opcode::ADD, false);
    test_compile_infix("-", opcode::SUB, false);
    test_compile_infix("*", opcode::MUL, false);
    test_compile_infix("/", opcode::DIV, false);
    test_compile_infix("%", opcode::MOD, false);
    test_compile_infix("==", opcode::EQUAL, false);
    test_compile_infix("!=", opcode::NOT_EQUAL, false);
    test_compile_infix("<", opcode::LESS_THAN, false);
    test_compile_infix("<=", opcode::LESS_THAN_EQUAL, false);
    test_compile_infix(">", opcode::LESS_THAN, true);
    test_compile_infix(">=", opcode::LESS_THAN_EQUAL, true);
}

#[test]
fn prefix_expressions() {
    let code = test_compile("-5;").unwrap();

    assert_eq!(code.instructions, vec![
        opcode::CONSTANT, 0, 0,
        opcode::MINUS,
        opcode::POP,
        opcode::RETURN_VALUE,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(5),
    ]);
}

#[test]
fn if_expressions() {
    let code = test_compile("if (1 == 1) { 10 }; 9;").unwrap();

    assert_eq!(code.instructions, vec![
        opcode::CONSTANT, 0, 0,
        opcode::CONSTANT, 0, 1,
        opcode::EQUAL,
        opcode::JUMP_IF_FALSE, 0, 6,
        opcode::CONSTANT, 0, 2,
        opcode::JUMP, 0, 1,
        opcode::NULL,
        opcode::POP,
        opcode::CONSTANT, 0, 3,
        opcode::POP,
        opcode::RETURN_VALUE,
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
        opcode::TRUE,
        opcode::JUMP_IF_FALSE, 0, 6,
        opcode::CONSTANT, 0, 0,
        opcode::JUMP, 0, 3,
        opcode::CONSTANT, 0, 1,
        opcode::POP,
        opcode::RETURN_VALUE,
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
        opcode::TRUE,
        opcode::JUMP_IF_FALSE, 0, 6,
        opcode::CONSTANT, 0, 0,
        opcode::JUMP, 0, 13,
        opcode::TRUE,
        opcode::JUMP_IF_FALSE, 0, 6,
        opcode::CONSTANT, 0, 1,
        opcode::JUMP, 0, 3,
        opcode::CONSTANT, 0, 2,
        opcode::POP,
        opcode::RETURN_VALUE,
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
        opcode::CONSTANT, 0, 0,
        opcode::SET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::CONSTANT, 0, 1,
        opcode::SET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::GET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::RETURN_VALUE,
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
        opcode::CONSTANT, 0, 0,
        opcode::SET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::GET_GLOBAL, 0, 1,
        opcode::CONSTANT, 0, 1,
        opcode::ADD,
        opcode::SET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::RETURN_VALUE,
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
        opcode::CONSTANT, 0, 0,
        opcode::SET_GLOBAL, 0, 1, // need to change when block scope
        opcode::POP,
        opcode::RETURN_VALUE,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(12),
    ]);
}

#[test]
fn function_expressions() {
    let code = test_compile("ten := fn() { 10 };").unwrap();

    assert_eq!(code.instructions, vec![
        opcode::CONSTANT, 0, 1,
        opcode::SET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::RETURN_VALUE,

        // ten function instructions
        opcode::CONSTANT, 0, 0,
        opcode::RETURN_VALUE,
    ]);

    assert_eq!(code.constants, vec![
        Object::Integer(10),
        // Object::Function(Function {
        //     pointer: 8,
        //     locals_count: 0,
        //     arity: 0
        // })
    ]);
}

#[test]
fn function_with_args_expressions() {
    let code = test_compile("add := fn(a, b) { a + b }; three := add(1, 2);").unwrap();

    assert_eq!(code.instructions, vec![
        opcode::CONSTANT, 0, 0,
        opcode::SET_GLOBAL, 0, 1,
        opcode::POP,
        opcode::CONSTANT, 0, 1,
        opcode::CONSTANT, 0, 2,
        opcode::GET_GLOBAL, 0, 1,
        opcode::CALL,
        opcode::SET_GLOBAL, 0, 2,
        opcode::POP,
        opcode::RETURN_VALUE,

        // add function instructions
        opcode::GET_LOCAL, 0,
        opcode::GET_LOCAL, 1,
        opcode::ADD,
        opcode::RETURN_VALUE,
    ]);

    assert_eq!(code.constants, vec![
        // Object::Function(Function {
        //     pointer: 22,
        //     locals_count: 2,
        //     arity: 2,
        // }),
        Object::Integer(2),
        Object::Integer(1),
    ]);
}
