use crate::{
    ast::{self, Node, Statement},
    token,
};

pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Box<dyn ast::Statement>>,
}

impl ToString for BlockStatement {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for statement in &self.statements {
            result += &statement.to_string();
        }

        result
    }
}

impl Node for BlockStatement {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}
