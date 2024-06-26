#![allow(dead_code)]

use belalang_core::{
    ast,
    lexer::{self, Lexer},
    parser,
    token::Token,
};

pub fn test_parse(input: &str) -> ast::Program {
    let input = input.as_bytes().into();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);

    parser.parse_program().expect("parser errors")
}

pub fn test_parse_to_string(input: &str, expected: &str) {
    let program = test_parse(input);
    assert_eq!(program.to_string(), expected);
}

pub fn test_tokens(input: &str, tokens: Vec<Token>) {
    let mut lexer = Lexer::new(input.as_bytes());

    for expected in tokens {
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, expected);
    }
}

#[macro_export]
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        if let $variant(x) = $value {
            x
        } else {
            panic!("unmatching variant! got={}", stringify!($variant));
        }
    };
}

#[macro_export]
macro_rules! ident_has_name {
    ($value:expr, $expected:expr) => {
        assert_eq!($value.value, $expected);
        assert_eq!($value.token.to_string(), $expected.to_string());
    };
}

#[macro_export]
macro_rules! expr_variant {
    ($value:expr, $variant:path = $expected:expr) => {
        let v = as_variant!($value, $variant);

        assert_eq!(v.value, $expected);
        assert_eq!(v.token.to_string(), $expected.to_string());
    };
    ($value:expr, Infix => ($left_variant:path = $left:expr, $op:expr, $right_variant:path = $right:expr)) => {
        let v = as_variant!($value, ast::Expression::Infix);

        expr_variant!(&*v.left, $left_variant = $left);
        expr_variant!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op);
    };
    ($value: expr, Prefix => ($op:expr, $right_variant:path = $right:expr)) => {
        let v = as_variant!($value, ast::Expression::Prefix);

        expr_variant!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op);
    };
}
