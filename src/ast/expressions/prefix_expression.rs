use super::Expression;
use crate::token;

pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "PrefixExpression(op={}, right={})",
            self.operator,
            self.right.to_string(),
        ))
    }
}
