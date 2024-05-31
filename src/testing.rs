macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

pub(crate) use as_variant;

macro_rules! stringify {
    ($value:expr, $expected:expr) => {
        assert_eq!($value.to_string(), $expected);
    };
}

pub(crate) use stringify;

macro_rules! ident {
    ($value:expr, $expected:expr) => {
        assert_eq!($value.value, $expected);
        assert_eq!($value.token.to_string(), $expected.to_string());
    };
}

pub(crate) use ident;

macro_rules! expr {
    ($value:expr, $variant:path = $expected:expr) => {
        let v = testing::as_variant!($value, $variant).expect("variant not match");

        assert_eq!(v.value, $expected);
        assert_eq!(v.token.to_string(), $expected.to_string());
    };
}

pub(crate) use expr;

macro_rules! stmt {
    ($value:expr, $variant:path = $expected:expr) => {
        let v = testing::as_variant!($value, $variant).expect("variant not match");

        assert_eq!(v.to_string(), $expected);
    };
}

pub(crate) use stmt;

macro_rules! infix {
    ($value:expr, $left_variant:path = $left:expr, $op:expr, $right_variant:path = $right:expr) => {
        let v = testing::as_variant!($value, ast::Expression::InfixExpression)
            .expect("not a(n) ast::Expression::InfixExpression");

        testing::expr!(&*v.left, $left_variant = $left);
        testing::expr!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op.to_string());
    };
}

pub(crate) use infix;

use crate::{evaluator, lexer, object, parser};

pub fn test_eval(input: String) -> object::Object {
    let input = input.as_bytes().into();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program().expect("parser errors");

    return evaluator::eval_program(program);
}

macro_rules! eval {
    ($input:expr, $variant:path = $expected:expr) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            $variant(obj) => assert_eq!(obj.value, $expected),
            _ => panic!("incorrect object type. got={}", evaluated),
        }
    };
    ($input:expr, $variant:path) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            $variant(_) => {},
            _ => panic!("object not null. got={}", evaluated),
        }
    };
}

pub(crate) use eval;
