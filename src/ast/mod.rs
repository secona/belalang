mod expressions;
mod program;
mod statements;

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

impl_downcast!(Expression);
