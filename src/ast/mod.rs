mod expression_statement;
mod identifier;
mod let_statement;
mod program;
mod return_statement;

pub use expression_statement::ExpressionStatement;
pub use identifier::Identifier;
pub use let_statement::LetStatement;
pub use program::Program;
pub use return_statement::ReturnStatement;

use crate::token;

pub trait Node: ToString {
    fn token(&self) -> Option<&token::Token>;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}
