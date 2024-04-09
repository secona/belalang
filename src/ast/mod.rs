mod expressions;
mod program;
mod statements;

use std::fmt::Debug;

pub use expressions::*;
pub use program::Program;
pub use statements::*;

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

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl_downcast!(Expression);
