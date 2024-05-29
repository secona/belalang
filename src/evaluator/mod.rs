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
        _ => object::Object::Null(object::Null {}),
    }
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
            _ => object::Object::Null(object::Null {}),
        },
        ast::Node::Statement(node) => match node {
            ast::Statement::ExpressionStatement(node) => {
                eval(ast::Node::Expression(node.expression))
            }
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
    }

    #[test]
    fn boolean() {
        testing::eval!("true", object::Object::Boolean = true);
        testing::eval!("false", object::Object::Boolean = false);
        testing::eval!("!true", object::Object::Boolean = false);
        testing::eval!("!!false", object::Object::Boolean = false);
        testing::eval!("!!!false", object::Object::Boolean = true);
        testing::eval!("!!!!true", object::Object::Boolean = true);
    }
}
