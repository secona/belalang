mod expression_statement;
mod identifier;
mod integer_literal;
mod let_statement;
mod prefix_expression;
mod program;
mod return_statement;

pub use expression_statement::ExpressionStatement;
pub use identifier::Identifier;
pub use integer_literal::IntegerLiteral;
pub use let_statement::LetStatement;
pub use prefix_expression::PrefixExpression;
pub use program::Program;
pub use return_statement::ReturnStatement;

use crate::token;
use downcast_rs::{impl_downcast, Downcast};

pub trait Node: ToString {
    fn token(&self) -> Option<&token::Token>;
}

pub trait Statement: Node + Downcast {
    fn statement_node(&self);
}

impl_downcast!(Statement);

pub trait Expression: Node + Downcast {
    fn expression_node(&self);
}

impl_downcast!(Expression);
