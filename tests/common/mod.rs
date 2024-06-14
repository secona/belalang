#![allow(dead_code)]

use belalang::{ast, evaluator, lexer, parser};

pub fn test_parse(input: &str) -> ast::Program {
    let input = input.as_bytes().into();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);

    parser.parse_program().expect("parser errors")
}

pub fn test_eval(
    input: String,
) -> Result<evaluator::object::Object, evaluator::error::EvaluatorError> {
    let input = input.as_bytes().into();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program().expect("parser errors");

    let mut ev = evaluator::Evaluator::default();
    return ev.eval_program(program);
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
}

#[macro_export]
macro_rules! eval {
    ($input:expr, $variant:path = $expected:expr) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            Ok($variant(value)) => assert_eq!(value, $expected),
            Ok(unexpected) => panic!("got unexpected object. got={}", unexpected),
            Err(err) => panic!("got errors instead. got={}", err),
        }
    };
    ($input:expr, $variant:pat) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            Ok(obj) => matches!(obj, $variant),
            Err(err) => panic!("got errors instead. got={}", err),
        }
    };
    ($input:expr, Err => $expected:expr) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            Ok(unexpected) => panic!("got ok instead. got={}", unexpected),
            Err(err) => assert_eq!(err.to_string(), $expected),
        }
    };
}