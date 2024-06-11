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
    Block(BlockStatement),
    Expression(ExpressionStatement),
    Return(ReturnStatement),
    Var(Var),
    While(WhileStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::Block(v) => v.to_string(),
            Statement::Expression(v) => v.to_string(),
            Statement::Return(v) => v.to_string(),
            Statement::Var(v) => v.to_string(),
            Statement::While(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
