use super::Expression;
use crate::token;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl std::fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "InfixExpression(left={} operator={} right={})",
            self.left.to_string(),
            self.operator,
            self.right.to_string(),
        ))
    }
}
