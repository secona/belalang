#![allow(dead_code)]

pub mod error;

use crate::{
    ast::{self, Expression, Node, Statement},
    object::{self, Object},
};

use self::error::EvaluatorError;

pub struct Evaluator {
    program: ast::Program,
    env: object::Environment,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self {
            env: object::Environment::default(),
            program: ast::Program {
                statements: Vec::new(),
            },
        }
    }
}

impl Evaluator {
    pub fn new(program: ast::Program) -> Self {
        Self {
            program,
            env: object::Environment::default(),
        }
    }

    pub fn evaluate(&mut self) -> Result<Object, EvaluatorError> {
        let mut statements = Vec::with_capacity(self.program.statements.len());
        std::mem::swap(&mut statements, &mut self.program.statements);

        self.evaluate_statements(statements)
    }

    pub fn evaluate_statements(
        &mut self,
        statements: Vec<Statement>,
    ) -> Result<Object, EvaluatorError> {
        let mut result: Object = Object::Null;

        for statement in statements {
            result = self.eval_statement(statement)?;
        }

        Ok(result)
    }

    fn eval(&mut self, node: Node) -> Result<Object, EvaluatorError> {
        match node {
            Node::Expression(expression) => self.eval_expression(expression),
            Node::Statement(statement) => self.eval_statement(statement),
            _ => Ok(Object::Null),
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Result<Object, EvaluatorError> {
        match expression {
            Expression::IntegerLiteral(int_lit) => Ok(Object::Integer(int_lit.value)),
            Expression::BooleanExpression(bool_expr) => Ok(Object::Boolean(bool_expr.value)),
            Expression::PrefixExpression(node) => {
                let right = self.eval_expression(*node.right)?;

                match node.operator.as_str() {
                    "!" => match right {
                        Object::Boolean(value) => Ok(Object::Boolean(!value)),
                        _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    "-" => match right {
                        Object::Integer(value) => Ok(Object::Integer(-value)),
                        _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                }
            }
            Expression::InfixExpression(infix_expr) => {
                let left = self.eval_expression(*infix_expr.left)?;
                let right = self.eval_expression(*infix_expr.right)?;

                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        match infix_expr.operator.as_str() {
                            "+" => Ok(Object::Integer(l + r)),
                            "-" => Ok(Object::Integer(l - r)),
                            "*" => Ok(Object::Integer(l * r)),
                            "/" => Ok(Object::Integer(l / r)),
                            "<" => Ok(Object::Boolean(l < r)),
                            ">" => Ok(Object::Boolean(l > r)),
                            "==" => Ok(Object::Boolean(l == r)),
                            "!=" => Ok(Object::Boolean(l != r)),
                            _ => Err(EvaluatorError::UnknownInfixOperator(
                                left,
                                infix_expr.operator,
                                right,
                            )),
                        }
                    }
                    (_, _) => match infix_expr.operator.as_str() {
                        "==" => Ok(Object::Boolean(left == right)),
                        "!=" => Ok(Object::Boolean(left != right)),
                        _ => Err(EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                }
            }
            Expression::IfExpression(expr) => {
                let condition = self.eval_expression(*expr.condition)?;

                if let Object::Boolean(value) = condition {
                    if value == true {
                        return self
                            .eval_statement(Statement::BlockStatement(expr.consequence));
                    }
                }

                if let Some(block_statement) = expr.alternative {
                    return self.eval_statement(Statement::BlockStatement(block_statement));
                }

                Ok(Object::Null)
            }
            Expression::CallExpression(_) => todo!(),
            Expression::FunctionLiteral(_) => todo!(),
            Expression::Identifier(_) => todo!(),
        }
    }

    fn eval_statement(&mut self, statement: Statement) -> Result<Object, EvaluatorError> {
        match statement {
            Statement::ExpressionStatement(node) => self.eval_expression(node.expression),
            Statement::BlockStatement(_) => todo!(),
            Statement::LetStatement(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
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
