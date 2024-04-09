use crate::{
    ast::{Expression, Node},
    token,
};

pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            (*self.left).to_string(),
            self.operator,
            (*self.right).to_string(),
        )
    }
}

impl Node for InfixExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}
