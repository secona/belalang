mod block_statement;
mod expression_statement;
mod return_statement;
mod var;
mod while_statement;

pub use block_statement::*;
pub use expression_statement::*;
pub use return_statement::*;
pub use var::*;
pub use while_statement::*;

#[derive(Debug, Clone)]
pub enum Statement {
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    ReturnStatement(ReturnStatement),
    Var(Var),
    WhileStatement(WhileStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::BlockStatement(v) => v.to_string(),
            Statement::ExpressionStatement(v) => v.to_string(),
            Statement::ReturnStatement(v) => v.to_string(),
            Statement::Var(v) => v.to_string(),
            Statement::WhileStatement(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
