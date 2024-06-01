#![allow(dead_code)]

pub mod error;

use std::{cell::RefCell, rc::Rc};

use crate::{ast, object};

pub struct Evaluator<'a> {
    program: ast::Program,
    env: Rc<RefCell<object::Environment<'a>>>,
}

impl Default for Evaluator<'_> {
    fn default() -> Self {
        Self {
            env: Rc::new(RefCell::new(object::Environment::default())),
            program: ast::Program {
                statements: Vec::new(),
            },
        }
    }
}

impl Evaluator<'_> {
    pub fn new(program: ast::Program) -> Self {
        Self {
            program,
            env: Rc::new(RefCell::new(object::Environment::default())),
        }
    }

    pub fn evaluate(&mut self) -> Result<object::Object, error::EvaluatorError> {
        let mut statements = Vec::with_capacity(self.program.statements.len());
        std::mem::swap(&mut statements, &mut self.program.statements);

        self.evaluate_statements(statements)
    }

    pub fn evaluate_statements(
        &mut self,
        statements: Vec<ast::Statement>,
    ) -> Result<object::Object, error::EvaluatorError> {
        let mut result: object::Object = object::Object::Null;

        for statement in statements {
            result = self.eval_statement(statement)?;

            if let object::Object::Return(r) = result {
                return Ok(*r);
            }
        }

        Ok(result)
    }

    fn eval(&mut self, node: ast::Node) -> Result<object::Object, error::EvaluatorError> {
        match node {
            ast::Node::Expression(expression) => self.eval_expression(expression),
            ast::Node::Statement(statement) => self.eval_statement(statement),
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_expression(
        &mut self,
        expression: ast::Expression,
    ) -> Result<object::Object, error::EvaluatorError> {
        match expression {
            ast::Expression::IntegerLiteral(int_lit) => Ok(object::Object::Integer(int_lit.value)),
            ast::Expression::BooleanExpression(bool_expr) => {
                Ok(object::Object::Boolean(bool_expr.value))
            }
            ast::Expression::PrefixExpression(node) => {
                let right = self.eval(ast::Node::Expression(*node.right))?;

                match node.operator.as_str() {
                    "!" => match right {
                        object::Object::Boolean(value) => Ok(object::Object::Boolean(!value)),
                        _ => Err(error::EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    "-" => match right {
                        object::Object::Integer(value) => Ok(object::Object::Integer(-value)),
                        _ => Err(error::EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    _ => Err(error::EvaluatorError::PrefixOperator(node.operator, right)),
                }
            }
            ast::Expression::InfixExpression(infix_expr) => {
                let left = self.eval(ast::Node::Expression(*infix_expr.left))?;
                let right = self.eval(ast::Node::Expression(*infix_expr.right))?;

                match (&left, &right) {
                    (object::Object::Integer(l), object::Object::Integer(r)) => {
                        match infix_expr.operator.as_str() {
                            "+" => Ok(object::Object::Integer(l + r)),
                            "-" => Ok(object::Object::Integer(l - r)),
                            "*" => Ok(object::Object::Integer(l * r)),
                            "/" => Ok(object::Object::Integer(l / r)),
                            "<" => Ok(object::Object::Boolean(l < r)),
                            ">" => Ok(object::Object::Boolean(l > r)),
                            "==" => Ok(object::Object::Boolean(l == r)),
                            "!=" => Ok(object::Object::Boolean(l != r)),
                            _ => Err(error::EvaluatorError::UnknownInfixOperator(
                                left,
                                infix_expr.operator,
                                right,
                            )),
                        }
                    }
                    (_, _) => match infix_expr.operator.as_str() {
                        "==" => Ok(object::Object::Boolean(left == right)),
                        "!=" => Ok(object::Object::Boolean(left != right)),
                        _ => Err(error::EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                }
            }
            ast::Expression::IfExpression(expr) => {
                let condition = self.eval(ast::Node::Expression(*expr.condition))?;

                if let object::Object::Boolean(value) = condition {
                    if value == true {
                        return self
                            .eval_statement(ast::Statement::BlockStatement(expr.consequence));
                    }
                }

                if let Some(block_statement) = expr.alternative {
                    return self.eval_statement(ast::Statement::BlockStatement(block_statement));
                }

                Ok(object::Object::Null)
            }
            ast::Expression::Identifier(node) => {
                let env = self.env.borrow();
                let value = env.get(&node.value);
                match value {
                    // TODO: change this clone. weird ahh implementation.
                    Some(value) => Ok(value.clone()),
                    None => Err(error::EvaluatorError::IdentifierNotFound(node.value)),
                }
            }
            _ => Ok(object::Object::Null),
        }
    }

    fn eval_statement(
        &mut self,
        statement: ast::Statement,
    ) -> Result<object::Object, error::EvaluatorError> {
        match statement {
            ast::Statement::ExpressionStatement(node) => {
                self.eval(ast::Node::Expression(node.expression))
            }
            ast::Statement::BlockStatement(block_statement) => {
                let mut result = object::Object::Null;

                for statement in block_statement.statements {
                    result = self.eval(ast::Node::Statement(statement))?;

                    if let object::Object::Return(_) = result {
                        return Ok(result);
                    }
                }

                Ok(result)
            }
            ast::Statement::ReturnStatement(return_statement) => {
                let value = self.eval(ast::Node::Expression(return_statement.return_value))?;
                Ok(object::Object::Return(Box::new(value)))
            }
            ast::Statement::LetStatement(let_statement) => {
                let value = self.eval(ast::Node::Expression(let_statement.value))?;

                self.env.borrow_mut().set(&let_statement.name.value, value);
                Ok(object::Object::Null)
            }
        }
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

    #[test]
    fn error_handling() {
        testing::eval!(
            "5 + true;",
            Err => "unknown operator: 5 + true"
        );
        testing::eval!(
            "if (1 < true) { return 10 }",
            Err => "unknown operator: 1 < true"
        );
        testing::eval!(
            "true + false",
            Err => "unknown operator: true + false"
        );
        testing::eval!(
            "4; true - true; 5",
            Err => "unknown operator: true - true"
        );
        testing::eval!(
            "b;",
            Err => "identifier not found: b"
        );
    }

    #[test]
    fn let_statements() {
        testing::eval!("let a = 5; a;", object::Object::Integer = 5);
        testing::eval!("let a = 5 * 10; a;", object::Object::Integer = 50);
        testing::eval!("let a = 10; let b = a; b;", object::Object::Integer = 10);

        testing::eval!(
            "let a = 1; let b = 1; let c = a + b * 2; c;",
            object::Object::Integer = 3
        );
    }
}
