#![allow(dead_code)]

use crate::{ast, token};

pub type InfixParser = fn(dyn ast::Expression) -> Box<dyn ast::Expression>;

pub fn lookup(tok: token::Token) -> Option<InfixParser> {
    match tok {
        _ => None
    }
}
