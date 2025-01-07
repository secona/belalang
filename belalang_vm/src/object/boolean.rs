#![allow(unused_variables)]

use std::fmt::Display;

use super::BelalangType;

#[derive(Debug)]
pub struct BelalangBoolean(bool);

impl Display for BelalangBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BelalangType for BelalangBoolean {
    fn type_name(&self) -> &str {
        "Boolean"
    }
}
