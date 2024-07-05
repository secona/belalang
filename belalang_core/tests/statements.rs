#[macro_use]
mod common;

use belalang_core::{ast, token};
use common::test_parse;

#[test]
fn block() {
    let program = test_parse("fn() { 12; 14; 1 + 2; };");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let f = as_variant!(&expr.expression, ast::Expression::Function);

    // +1 from implicit null expression at the end.
    assert_eq!(f.body.statements.len(), 4);

    // first statement
    let expr_0 = as_variant!(&f.body.statements[0], ast::Statement::Expression);

    let int_0 = as_variant!(&expr_0.expression, ast::Expression::Integer);

    assert_eq!(int_0.token, token::Token::Int("12".into()));
    assert_eq!(int_0.value, 12);

    // second statement
    let expr_1 = as_variant!(&f.body.statements[1], ast::Statement::Expression);

    let int_1 = as_variant!(&expr_1.expression, ast::Expression::Integer);

    assert_eq!(int_1.token, token::Token::Int("14".into()));
    assert_eq!(int_1.value, 14);

    // third statement
    let expr_2 = as_variant!(&f.body.statements[2], ast::Statement::Expression);

    expr_variant!(&expr_2.expression, Infix => (
        ast::Expression::Integer = 1,
        token::Token::Add,
        ast::Expression::Integer = 2
    ));
}

#[test]
fn r#return() {
    let program = test_parse("return 12;");

    assert_eq!(program.statements.len(), 1);

    let ret = as_variant!(&program.statements[0], ast::Statement::Return);

    assert_eq!(ret.token, token::Token::Return);

    let val = as_variant!(&ret.return_value, ast::Expression::Integer);

    assert_eq!(val.token, token::Token::Int("12".into()));
    assert_eq!(val.value, 12);
}

#[test]
fn r#while() {
    let program = test_parse("while (true) { 12; }");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::While);

    expr_variant!(&*stmt.condition, ast::Expression::Boolean = true);

    // +1 from implicit null expression at the end.
    assert_eq!(stmt.block.statements.len(), 2);

    let expr_0 = as_variant!(&stmt.block.statements[0], ast::Statement::Expression);

    expr_variant!(&expr_0.expression, ast::Expression::Integer = 12);
}
