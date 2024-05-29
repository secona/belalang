use super::Statement;
use crate::token;

pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        f.write_str(&format!("BlockStatement(statements=[{}])", statements))
    }
}
