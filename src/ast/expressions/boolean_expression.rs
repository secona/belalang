use crate::{
    ast::{Expression, Node},
    token,
};

pub struct BooleanExpression {
    pub token: token::Token,
    pub value: bool,
}

impl ToString for BooleanExpression {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Node for BooleanExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for BooleanExpression {
    fn expression_node(&self) {}
}
