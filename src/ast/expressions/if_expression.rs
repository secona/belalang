use crate::{
    ast::{BlockStatement, Expression, Node},
    token,
};

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl ToString for IfExpression {
    fn to_string(&self) -> String {
        let mut result = format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if let Some(stmt) = &self.alternative {
            result += &format!("else {}", stmt.to_string());
        }

        result
    }
}

impl Node for IfExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}
