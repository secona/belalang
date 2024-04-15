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
        format!(
            "{{ {} }}",
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        )
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
