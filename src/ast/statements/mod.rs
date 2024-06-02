mod block_statement;
mod expression_statement;
mod let_statement;
mod return_statement;

pub use block_statement::*;
pub use expression_statement::*;
pub use let_statement::*;
pub use return_statement::*;

#[derive(Debug, Clone)]
pub enum Statement {
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::BlockStatement(v) => v.to_string(),
            Statement::ExpressionStatement(v) => v.to_string(),
            Statement::LetStatement(v) => v.to_string(),
            Statement::ReturnStatement(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
