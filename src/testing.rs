#![allow(dead_code)]

use crate::ast;

pub struct ToStringTest<T: ToString> {
    pub obj: T,
    pub exp: String,
}

impl<T: ToString> ToStringTest<T> {
    pub fn test(&self) {
        assert_eq!(self.obj.to_string(), self.exp);
    }
}

macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

pub(crate) use as_variant;

macro_rules! expression {
    ($value:expr, $expected:expr) => {
        assert_eq!($value.value, $expected);
        assert_eq!($value.token.to_string(), $expected.to_string());
    };
}

pub(crate) use expression;

macro_rules! literal {
    ($value:expr, $variant:path = $expected:expr) => {
        let v = testing::as_variant!($value, $variant).expect("variant not match");

        assert_eq!(v.value, $expected);
        assert_eq!(v.token.to_string(), $expected.to_string());
    };
}

pub(crate) use literal;

pub fn test_identifier(expr: &ast::Expression, value: &'static str) {
    let ident = as_variant!(expr, ast::Expression::Identifier)
        .expect("not a(n) ast::Expression::Identifier");

    assert_eq!(ident.value, value);
    assert_eq!(ident.token.to_string(), value);
}

pub fn test_integer_literal(expr: &ast::Expression, value: i64) {
    let integer = as_variant!(expr, ast::Expression::IntegerLiteral)
        .expect("not a(n) ast::Expression::Identifier");

    assert_eq!(integer.value, value);
    assert_eq!(integer.token.to_string(), value.to_string());
}

pub fn test_literal_expression(expr: &ast::Expression, exp: Expected) {
    match exp {
        Expected::Ident(s) => test_identifier(expr, s),
        Expected::Integer(i) => test_integer_literal(expr, i),
    }
}

pub enum Expected {
    Ident(&'static str),
    Integer(i64),
}

macro_rules! infix {
    ($value:expr, $left_variant:path = $left:expr, $op:expr, $right_variant:path = $right:expr) => {
        let v = testing::as_variant!($value, ast::Expression::InfixExpression)
            .expect("not a(n) ast::Expression::InfixExpression");

        testing::literal!(&*v.left, $left_variant = $left);
        testing::literal!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op.to_string());
    };
}

pub(crate) use infix;

pub fn test_infix_expression(
    expr: &ast::Expression,
    left: Expected,
    op: &'static str,
    right: Expected,
) {
    let infix_expr = as_variant!(expr, ast::Expression::InfixExpression)
        .expect("not a(n) ast::Expressoin::InfixExpression");

    test_literal_expression(&infix_expr.left, left);
    test_literal_expression(&infix_expr.right, right);
    assert_eq!(infix_expr.operator, op.to_string());
}
