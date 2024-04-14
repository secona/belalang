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

pub fn test_identifier(expr: &dyn ast::Expression, value: &'static str) {
    let ident = expr
        .downcast_ref::<ast::Identifier>()
        .expect("not a(n) ast::Identifier");

    assert_eq!(ident.value, value);
    assert_eq!(ident.token.to_string(), value);
}

pub fn test_integer_literal(expr: &dyn ast::Expression, value: i64) {
    let integer = expr
        .downcast_ref::<ast::IntegerLiteral>()
        .expect("not a(n) ast::IntegerLiteral");

    assert_eq!(integer.value, value);
    assert_eq!(integer.token.to_string(), value.to_string());
}

pub fn test_literal_expression(expr: &dyn ast::Expression, exp: Expected) {
    match exp {
        Expected::Ident(s) => test_identifier(expr, s),
        Expected::Integer(i) => test_integer_literal(expr, i),
    }
}

pub enum Expected {
    Ident(&'static str),
    Integer(i64),
}

pub fn test_infix_expression(
    expr: &dyn ast::Expression,
    left: Expected,
    op: &'static str,
    right: Expected,
) {
    let infix_expr = expr
        .downcast_ref::<ast::InfixExpression>()
        .expect("not a(n) ast::InfixExpression");

    test_literal_expression(infix_expr.left.as_ref(), left);
    test_literal_expression(infix_expr.right.as_ref(), right);
    assert_eq!(infix_expr.operator, op.to_string());
}
