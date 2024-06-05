mod block_statement;
mod expression_statement;
mod return_statement;
mod var_assign;
mod var_declare;

pub use block_statement::*;
pub use expression_statement::*;
pub use return_statement::*;
pub use var_assign::*;
pub use var_declare::*;

#[derive(Debug, Clone)]
pub enum Statement {
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    ReturnStatement(ReturnStatement),
    VarAssign(VarAssign),
    VarDeclare(VarDeclare),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::BlockStatement(v) => v.to_string(),
            Statement::ExpressionStatement(v) => v.to_string(),
            Statement::ReturnStatement(v) => v.to_string(),
            Statement::VarAssign(v) => v.to_string(),
            Statement::VarDeclare(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
