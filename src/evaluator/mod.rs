#![allow(dead_code)]

use crate::{ast, object};

pub fn eval_program(program: ast::Program) -> object::Object {
    let mut result: object::Object = object::Object::Null(object::Null {});

    for statement in program.statements {
        result = eval(ast::Node::Statement(statement));
    }

    result
}

fn eval_prefix_expression(operator: String, right: ast::Expression) -> object::Object {
    match operator.as_str() {
        "!" => match right {
            ast::Expression::BooleanExpression(bool) => {
                object::Object::Boolean(object::Boolean { value: !bool.value })
            }
            _ => object::Object::Null(object::Null {  })
        },
        _ => object::Object::Null(object::Null {  })
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
                eval_prefix_expression(node.operator, *node.right)
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
    use crate::{lexer, object, parser};

    fn test_eval(input: String) -> object::Object {
        let input = input.as_bytes().into();
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().expect("parser errors");

        return super::eval_program(program);
    }

    #[test]
    fn eval_integer_expression() {
        let input = String::from("5");
        let expected = 5;

        let evaluated = test_eval(input);

        match evaluated {
            object::Object::Integer(obj) => assert_eq!(obj.value, expected),
            _ => {
                panic!("object not Integer. got={}", evaluated);
            }
        }
    }

    #[test]
    fn eval_boolean_expression() {
        let input = String::from("true");
        let expected = true;

        let evaluated = test_eval(input);

        match evaluated {
            object::Object::Boolean(obj) => assert_eq!(obj.value, expected),
            _ => {
                panic!("object not Boolean. got={}", evaluated);
            }
        }
    }

    #[test]
    fn eval_bang_boolean_expression() {
        let input = String::from("!true");
        let expected = false;
        
        let evaluated = test_eval(input);

        match evaluated {
            object::Object::Boolean(obj) => assert_eq!(obj.value, expected),
            _ => {
                panic!("object not Boolean. got={}", evaluated);
            }
        }
    }
}
