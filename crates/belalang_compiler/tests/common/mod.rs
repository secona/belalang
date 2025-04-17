#![allow(dead_code)]

use belalang_compiler::ast;
use belalang_compiler::ast::Parser;
use belalang_compiler::tokens::Lexer;

pub fn test_parse(input: &str) -> ast::Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    parser.parse_program().expect("parser errors")
}

pub fn test_parse_to_string(input: &str, expected: &str) {
    let program = test_parse(input);
    assert_eq!(program.to_string(), expected);
}

#[macro_export]
macro_rules! as_variant {
    ($value:expr_2021, $variant:path) => {
        if let $variant(x) = $value {
            x
        } else {
            panic!("unmatching variant! got={}", stringify!($variant));
        }
    };
}

#[macro_export]
macro_rules! ident_has_name {
    ($value:expr_2021, $expected:expr_2021) => {
        assert_eq!($value.value, $expected);
        assert_eq!($value.token.to_string(), $expected.to_string());
    };
}

#[macro_export]
macro_rules! expr_variant {
    ($value:expr_2021, $variant:path = $expected:expr_2021) => {
        let v = as_variant!($value, $variant);

        assert_eq!(v.value, $expected);
        assert_eq!(v.token.to_string(), $expected.to_string());
    };
    ($value:expr_2021, Infix => ($left_variant:path = $left:expr_2021, $op:expr_2021, $right_variant:path = $right:expr_2021)) => {
        let v = as_variant!($value, ast::Expression::Infix);

        expr_variant!(&*v.left, $left_variant = $left);
        expr_variant!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op);
    };
    ($value: expr_2021, Prefix => ($op:expr_2021, $right_variant:path = $right:expr_2021)) => {
        let v = as_variant!($value, ast::Expression::Prefix);

        expr_variant!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op);
    };
}
