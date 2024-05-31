#![allow(dead_code)]

use crate::{ast, object};

pub fn eval_program(program: ast::Program) -> object::Object {
    let mut result: object::Object = object::Object::Null;

    for statement in program.statements {
        result = eval(ast::Node::Statement(statement));

        if let object::Object::Return(r) = result {
            return *r;
        }
    }

    result
}

fn eval_prefix_expression(operator: String, right: object::Object) -> object::Object {
    match operator.as_str() {
        "!" => match right {
            object::Object::Boolean(value) => object::Object::Boolean(!value),
            _ => object::Object::Null,
        },
        "-" => match right {
            object::Object::Integer(value) => object::Object::Integer(-value),
            _ => object::Object::Null,
        },
        _ => object::Object::Null,
    }
}

fn eval_int_infix_expression(operator: String, left: i64, right: i64) -> object::Object {
    match operator.as_str() {
        "+" => object::Object::Integer(left + right),
        "-" => object::Object::Integer(left - right),
        "*" => object::Object::Integer(left * right),
        "/" => object::Object::Integer(left / right),
        "<" => object::Object::Boolean(left < right),
        ">" => object::Object::Boolean(left > right),
        "==" => object::Object::Boolean(left == right),
        "!=" => object::Object::Boolean(left != right),
        _ => object::Object::Null,
    }
}

fn eval_infix_expression(
    operator: String,
    left: object::Object,
    right: object::Object,
) -> object::Object {
    if let (object::Object::Integer(l), object::Object::Integer(r)) = (&left, &right) {
        return eval_int_infix_expression(operator, *l, *r);
    }

    match operator.as_str() {
        "==" => object::Object::Boolean(left == right),
        "!=" => object::Object::Boolean(left != right),
        _ => object::Object::Null,
    }
}

fn eval_if_expression(expr: ast::IfExpression) -> object::Object {
    let condition = eval(ast::Node::Expression(*expr.condition));

    if let object::Object::Boolean(value) = condition {
        if value == true {
            return eval(ast::Node::Statement(ast::Statement::BlockStatement(
                expr.consequence,
            )));
        }
    }

    if let Some(block_statement) = expr.alternative {
        return eval(ast::Node::Statement(ast::Statement::BlockStatement(
            block_statement,
        )));
    }

    object::Object::Null
}

fn eval_block_statement(block_statement: ast::BlockStatement) -> object::Object {
    let mut result = object::Object::Null;

    for statement in block_statement.statements {
        result = eval(ast::Node::Statement(statement));

        if let object::Object::Return(_) = result {
            return result;
        }
    }

    result
}

pub fn eval(node: ast::Node) -> object::Object {
    match node {
        ast::Node::Expression(node) => match node {
            ast::Expression::IntegerLiteral(int_lit) => object::Object::Integer(int_lit.value),
            ast::Expression::BooleanExpression(bool_expr) => {
                object::Object::Boolean(bool_expr.value)
            }
            ast::Expression::PrefixExpression(node) => {
                let right = eval(ast::Node::Expression(*node.right));
                eval_prefix_expression(node.operator, right)
            }
            ast::Expression::InfixExpression(node) => {
                let left = eval(ast::Node::Expression(*node.left));
                let right = eval(ast::Node::Expression(*node.right));
                eval_infix_expression(node.operator, left, right)
            }
            ast::Expression::IfExpression(node) => eval_if_expression(node),
            _ => object::Object::Null,
        },
        ast::Node::Statement(node) => match node {
            ast::Statement::ExpressionStatement(node) => {
                eval(ast::Node::Expression(node.expression))
            }
            ast::Statement::BlockStatement(block_statement) => {
                eval_block_statement(block_statement)
            }
            ast::Statement::ReturnStatement(return_statement) => {
                let value = eval(ast::Node::Expression(return_statement.return_value));
                object::Object::Return(Box::new(value))
            }
            _ => object::Object::Null,
        },
        _ => object::Object::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::{object, testing};

    #[test]
    fn integer() {
        testing::eval!("5", object::Object::Integer = 5);
        testing::eval!("1209", object::Object::Integer = 1209);

        testing::eval!("-123", object::Object::Integer = -123);
        testing::eval!("--123", object::Object::Integer = 123);
        testing::eval!("---123", object::Object::Integer = -123);

        testing::eval!("12 * 3", object::Object::Integer = 36);
        testing::eval!("12 / 3 + 1", object::Object::Integer = 5);
        testing::eval!("(5 + 1) / 2", object::Object::Integer = 3);
        testing::eval!("5 * -2", object::Object::Integer = -10);
        testing::eval!("-5 * -2", object::Object::Integer = 10);
    }

    #[test]
    fn boolean() {
        testing::eval!("true", object::Object::Boolean = true);
        testing::eval!("false", object::Object::Boolean = false);

        testing::eval!("!true", object::Object::Boolean = false);
        testing::eval!("!!false", object::Object::Boolean = false);
        testing::eval!("!!!false", object::Object::Boolean = true);
        testing::eval!("!!!!true", object::Object::Boolean = true);

        testing::eval!("1 == 1", object::Object::Boolean = true);
        testing::eval!("2 != 1", object::Object::Boolean = true);
        testing::eval!("2 == 1", object::Object::Boolean = false);
        testing::eval!("2 * 4 == 8", object::Object::Boolean = true);
        testing::eval!("-1 < 1", object::Object::Boolean = true);
        testing::eval!("1 < 1", object::Object::Boolean = false);
        testing::eval!("1 - 2 < 1", object::Object::Boolean = true);
        testing::eval!("1 + 2 > 1", object::Object::Boolean = true);

        testing::eval!("true == true", object::Object::Boolean = true);
        testing::eval!("false == false", object::Object::Boolean = true);
        testing::eval!("true == false", object::Object::Boolean = false);
        testing::eval!("true != false", object::Object::Boolean = true);
        testing::eval!("1 < 2 == true", object::Object::Boolean = true);
    }

    #[test]
    fn if_expressions() {
        testing::eval!("if (true) { 1 }", object::Object::Integer = 1);
        testing::eval!("if (false) { 1 } else { 2 }", object::Object::Integer = 2);

        testing::eval!(
            "if (1 < 2) { true } else { false }",
            object::Object::Boolean = true
        );
        testing::eval!(
            "if (1 > 2) { true } else { false }",
            object::Object::Boolean = false
        );
        testing::eval!(
            "if (1 + 2 == 3) { 1 + 2 } else { false }",
            object::Object::Integer = 3
        );
        testing::eval!(
            "if (1) { true } else { false }",
            object::Object::Boolean = false
        );

        testing::eval!("if (false) { true }", object::Object::Null);
    }

    #[test]
    fn return_statements() {
        testing::eval!(
            "if (true) { return 10; 9; } else { return 10; }",
            object::Object::Integer = 10
        );
        testing::eval!(
            "if (false) { 0; } else { return 1; 10; }",
            object::Object::Integer = 1
        );
        testing::eval!(
            "
if (10 > 1) {
    if (10 > 1) {
        return true;
    }

    return false;
}",
            object::Object::Boolean = true
        );
    }
}
