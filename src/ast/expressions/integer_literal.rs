use crate::token;
use crate::ast::{Expression, Node};

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Node for IntegerLiteral {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        
    }
}
