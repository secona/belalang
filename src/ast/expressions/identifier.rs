use crate::token;
use crate::ast::{Expression, Node};

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Node for Identifier {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

