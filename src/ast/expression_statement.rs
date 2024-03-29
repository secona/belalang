use crate::token;

use super::{Expression, Node};

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Box<dyn Expression>,
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

impl Node for ExpressionStatement {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for ExpressionStatement {
    fn expression_node(&self) {}
}
