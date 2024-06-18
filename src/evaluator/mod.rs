pub mod builtins;
pub mod environment;
pub mod error;
pub mod object;

use crate::{
    ast::{BlockExpression, Expression, Node, Program, Statement},
    error::EvaluatorError,
    evaluator::{environment::Environment, object::Object},
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
                    Token::Not => match right {
                        Object::Boolean(value) => Ok(Object::Boolean(!value)),
                        _ => Err(EvaluatorError::PrefixOperator(node.operator, right)),
                    },
                    Token::Sub => match right {
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
                        Token::Add => Ok(Object::Integer(l + r)),
                        Token::Sub => Ok(Object::Integer(l - r)),
                        Token::Mul => Ok(Object::Integer(l * r)),
                        Token::Div => Ok(Object::Integer(l / r)),
                        Token::Mod => Ok(Object::Integer(l % r)),
                        Token::Lt => Ok(Object::Boolean(l < r)),
                        Token::Le => Ok(Object::Boolean(l <= r)),
                        Token::Gt => Ok(Object::Boolean(l > r)),
                        Token::Ge => Ok(Object::Boolean(l >= r)),
                        Token::Eq => Ok(Object::Boolean(l == r)),
                        Token::Ne => Ok(Object::Boolean(l != r)),
                        _ => Err(EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                    (Object::String(l), Object::String(r)) => match infix_expr.operator {
                        Token::Add => Ok(Object::String(format!("{} {}", l, r))),
                        _ => Err(EvaluatorError::UnknownInfixOperator(
                            left,
                            infix_expr.operator,
                            right,
                        )),
                    },
                    (_, _) => match infix_expr.operator {
                        Token::Eq => Ok(Object::Boolean(left == right)),
                        Token::Ne => Ok(Object::Boolean(left != right)),
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

                if let Some(expr) = expr.alternative {
                    return self.eval_expression(*expr);
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
                Token::ColonAssign => {
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
