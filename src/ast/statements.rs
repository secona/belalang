use crate::token;

use super::{BlockExpression, Expression, Identifier};

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Expression,
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", &self.expression)
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.return_value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {};", self.name, self.token, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub block: BlockExpression,
}

impl std::fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while ({}) {}", self.condition, self.block)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExpressionStatement),
    Return(ReturnStatement),
    Var(Var),
    While(WhileStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::Expression(v) => v.to_string(),
            Statement::Return(v) => v.to_string(),
            Statement::Var(v) => v.to_string(),
            Statement::While(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
