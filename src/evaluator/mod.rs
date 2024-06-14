pub mod builtins;
pub mod environment;
pub mod error;
pub mod object;

use crate::{
    ast::{BlockExpression, Expression, Node, Program, Statement},
    evaluator::{environment::Environment, error::EvaluatorError, object::Object},
    token::Token,
};

use self::builtins::Builtins;

pub struct Evaluator {
    env: Environment,
    builtins: Builtins,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self {
            env: Environment::default(),
            builtins: Builtins::default(),
        }
    }
}

impl Evaluator {
    pub fn new(builtins: Builtins) -> Self {
        Self {
            builtins,
            env: Environment::default(),
        }
    }

    pub fn eval(&mut self, node: Node) -> Result<Object, EvaluatorError> {
        match node {
            Node::Expression(expr) => self.eval_expression(expr),
            Node::Statement(stmt) => self.eval_statement(stmt),
            Node::Program(prog) => self.eval_program(prog),
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Object, EvaluatorError> {
        let mut result: Object = Object::Null;

        for statement in program.statements {
            result = self.eval_statement(statement)?;
        }

        Ok(result)
    }

    pub fn eval_expression(&mut self, expression: Expression) -> Result<Object, EvaluatorError> {
        match expression {
            Expression::Integer(int_lit) => Ok(Object::Integer(int_lit.value)),
            Expression::Boolean(bool_expr) => Ok(Object::Boolean(bool_expr.value)),
            Expression::String(s) => Ok(Object::String(s.value)),
            Expression::Null(_) => Ok(Object::Null),
            Expression::Prefix(node) => {
                let right = self.eval_expression(*node.right)?;

                match node.operator {
                    Token::Bang => match right {
                        Object::Boolean(value) => Ok(Object::Boolean(!value)),
                        _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    Token::Minus => match right {
                        Object::Integer(value) => Ok(Object::Integer(-value)),
                        _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                }
            }
            Expression::Infix(infix_expr) => {
                let left = self.eval_expression(*infix_expr.left)?;
                let right = self.eval_expression(*infix_expr.right)?;

                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => match infix_expr.operator {
                        Token::Plus => Ok(Object::Integer(l + r)),
                        Token::Minus => Ok(Object::Integer(l - r)),
                        Token::Asterisk => Ok(Object::Integer(l * r)),
                        Token::Slash => Ok(Object::Integer(l / r)),
                        Token::Percent => Ok(Object::Integer(l % r)),
                        Token::LT => Ok(Object::Boolean(l < r)),
                        Token::GT => Ok(Object::Boolean(l > r)),
                        Token::Eq => Ok(Object::Boolean(l == r)),
                        Token::NotEq => Ok(Object::Boolean(l != r)),
                        _ => Err(EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                    (Object::String(l), Object::String(r)) => match infix_expr.operator {
                        Token::Plus => Ok(Object::String(format!("{} {}", l, r))),
                        _ => Err(EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                    (_, _) => match infix_expr.operator {
                        Token::Eq => Ok(Object::Boolean(left == right)),
                        Token::NotEq => Ok(Object::Boolean(left != right)),
                        _ => Err(EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                }
            }
            Expression::If(expr) => {
                let condition = self.eval_expression(*expr.condition)?;

                if let Object::Boolean(true) = condition {
                    return self.eval_block(expr.consequence, self.env.capture());
                }

                if let Some(block_statement) = expr.alternative {
                    return self.eval_block(block_statement, self.env.capture());
                }

                Ok(Object::Null)
            }
            Expression::Call(call_expr) => {
                let function = self.eval_expression(*call_expr.function)?;
                let args = call_expr
                    .args
                    .into_iter()
                    .map(|arg| self.eval_expression(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                match function {
                    Object::Function { params, body, env } => {
                        let mut env = env.capture();
                        for (param, arg) in params.iter().zip(args) {
                            env.set(&param.value, arg);
                        }

                        match self.eval_block(body, env) {
                            Ok(v) => Ok(v),
                            Err(EvaluatorError::ReturningValue(v)) => Ok(v),
                            Err(e) => Err(e),
                        }
                    }
                    Object::Builtin(name) => Ok(self.builtins.call(name, args)),
                    _ => Err(EvaluatorError::NotAFunction()),
                }
            }
            Expression::Function(fn_lit) => Ok(Object::Function {
                params: fn_lit.params,
                body: fn_lit.body,
                env: self.env.clone(),
            }),
            Expression::Identifier(ident) => match self.env.get(&ident.value) {
                Some(value) => Ok(value.clone()),
                None => match self.builtins.has_fn(&ident.value) {
                    true => Ok(Object::Builtin(ident.value)),
                    false => Err(EvaluatorError::UnknownVariable(ident.value)),
                },
            },
            Expression::Block(block) => self.eval_block(block, self.env.capture()),
        }
    }

    pub fn eval_statement(&mut self, statement: Statement) -> Result<Object, EvaluatorError> {
        match statement {
            Statement::Expression(node) => self.eval_expression(node.expression),
            Statement::Return(return_stmt) => {
                let value = self.eval_expression(return_stmt.return_value)?;
                Err(EvaluatorError::ReturningValue(value))
            }
            Statement::Var(var) => match var.token {
                Token::Walrus => {
                    let name = &var.name.value;

                    if self.env.has_here(name) {
                        return Err(EvaluatorError::VariableRedeclaration(name.clone()));
                    }

                    if self.builtins.has_fn(name) {
                        return Err(EvaluatorError::OverwriteBuiltin(name.to_string()));
                    }

                    let value = self.eval_expression(var.value)?;
                    self.env.set(&var.name.value, value.clone());
                    Ok(value)
                }
                Token::Assign => {
                    let name = &var.name.value;

                    if self.builtins.has_fn(name) {
                        return Err(EvaluatorError::OverwriteBuiltin(name.to_string()));
                    }

                    if !self.env.has(name) {
                        return Err(EvaluatorError::UnknownVariable(name.clone()));
                    }

                    let value = self.eval_expression(var.value)?;
                    self.env.set(&var.name.value, value.clone());
                    Ok(value)
                }
                _ => Err(EvaluatorError::NotAFunction()),
            },
            Statement::While(stmt) => {
                while let Object::Boolean(true) = self.eval_expression(*stmt.condition.clone())? {
                    self.eval_block(stmt.block.clone(), self.env.capture())?;
                }

                Ok(Object::Null)
            }
        }
    }

    pub fn eval_block(
        &self,
        block: BlockExpression,
        env: Environment,
    ) -> Result<Object, EvaluatorError> {
        let mut result = Object::Null;
        let mut ev = Evaluator::default();
        ev.env = env;

        for statement in block.statements {
            result = ev.eval_statement(statement)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::object;
    use crate::testing;

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
        testing::eval!("5 % 2", object::Object::Integer = 1);
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
            Err => "unknown variable: b"
        );
    }

    #[test]
    fn variable_declaration() {
        testing::eval!("a := 5; a;", object::Object::Integer = 5);
        testing::eval!("a := 5 * 10; a;", object::Object::Integer = 50);
        testing::eval!("a := 10; b := a; b;", object::Object::Integer = 10);

        testing::eval!(
            "a := 1; b := 1; c := a + b * 2; c;",
            object::Object::Integer = 3
        );
    }
}
