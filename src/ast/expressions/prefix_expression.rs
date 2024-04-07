use crate::token;
use crate::ast::{Expression, Node};

pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator, (*self.right).to_string())
    }
}

impl Node for PrefixExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}
