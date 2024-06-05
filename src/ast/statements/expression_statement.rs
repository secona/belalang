use crate::ast::expressions::Expression;
use crate::token;

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

// tests for ExpressionStatement is already covered in all expressions
