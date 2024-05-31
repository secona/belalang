#![allow(dead_code)]

use crate::{ast, object};

pub fn eval_program(program: ast::Program) -> object::Object {
    let mut result: object::Object = object::Object::Null(object::Null {});

    for statement in program.statements {
        result = eval(ast::Node::Statement(statement));
    }

    result
}

fn eval_prefix_expression(operator: String, right: object::Object) -> object::Object {
    match operator.as_str() {
        "!" => match right {
            object::Object::Boolean(bool) => {
                object::Object::Boolean(object::Boolean { value: !bool.value })
            }
            _ => object::Object::Null(object::Null {}),
        },
        "-" => match right {
            object::Object::Integer(int) => {
                object::Object::Integer(object::Integer { value: -int.value })
            }
            _ => object::Object::Null(object::Null {}),
        },
        _ => object::Object::Null(object::Null {}),
    }
}

fn eval_int_infix_expression(
    operator: String,
    left: &object::Integer,
    right: &object::Integer,
) -> object::Object {
    let l = left.value;
    let r = right.value;

    match operator.as_str() {
        "+" => object::Object::Integer(object::Integer { value: l + r }),
        "-" => object::Object::Integer(object::Integer { value: l - r }),
        "*" => object::Object::Integer(object::Integer { value: l * r }),
        "/" => object::Object::Integer(object::Integer { value: l / r }),
        "<" => object::Object::Boolean(object::Boolean { value: l < r }),
        ">" => object::Object::Boolean(object::Boolean { value: l > r }),
        "==" => object::Object::Boolean(object::Boolean { value: l == r }),
        "!=" => object::Object::Boolean(object::Boolean { value: l != r }),
        _ => object::Object::Null(object::Null {}),
    }
}

fn eval_infix_expression(
    operator: String,
    left: object::Object,
    right: object::Object,
) -> object::Object {
    if let (object::Object::Integer(l), object::Object::Integer(r)) = (&left, &right) {
        return eval_int_infix_expression(operator, l, r);
    }

    match operator.as_str() {
        "==" => object::Object::Boolean(object::Boolean {
            value: left == right,
        }),
        "!=" => object::Object::Boolean(object::Boolean {
            value: left != right,
        }),
        _ => object::Object::Null(object::Null {}),
    }
}

fn eval_if_expression(expr: ast::IfExpression) -> object::Object {
    let condition = eval(ast::Node::Expression(*expr.condition));

    if let object::Object::Boolean(b) = condition {
        if b.value == true {
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

    object::Object::Null(object::Null {})
}

fn eval_statements(statements: Vec<ast::Statement>) -> object::Object {
    let mut res = object::Object::Null(object::Null {});

    for statement in statements {
        res = eval(ast::Node::Statement(statement));
    }

    res
}

pub fn eval(node: ast::Node) -> object::Object {
    match node {
        ast::Node::Expression(node) => match node {
            ast::Expression::IntegerLiteral(node) => {
                object::Object::Integer(object::Integer { value: node.value })
            }
            ast::Expression::BooleanExpression(node) => {
                object::Object::Boolean(object::Boolean { value: node.value })
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
            _ => object::Object::Null(object::Null {}),
        },
        ast::Node::Statement(node) => match node {
            ast::Statement::ExpressionStatement(node) => {
                eval(ast::Node::Expression(node.expression))
            }
            ast::Statement::BlockStatement(node) => eval_statements(node.statements),
            _ => object::Object::Null(object::Null {}),
        },
        _ => object::Object::Null(object::Null {}),
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
}
