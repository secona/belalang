use crate::ast::expressions::Expression;
use crate::token;

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Expression,
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ExpressionStatement(value={})",
            &self.expression.to_string()
        ))
    }
}
