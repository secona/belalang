#![allow(unused_variables)]

use std::{error::Error, fmt::Display};

use belalang_devel::ops::{And, Not, Or};
use belalang_devel::BelalangType;

#[derive(Debug, Clone)]
pub struct BelalangBoolean(pub bool);

impl Display for BelalangBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BelalangType for BelalangBoolean {
    fn type_name(&self) -> &str {
        "Boolean"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl And for BelalangBoolean {
    type Output = BelalangBoolean;
    type Rhs = BelalangBoolean;

    fn and(&self, other: &BelalangBoolean) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 && other.0))
    }
}

impl Or for BelalangBoolean {
    type Output = BelalangBoolean;
    type Rhs = BelalangBoolean;

    fn or(&self, other: &BelalangBoolean) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 || other.0))
    }
}

impl Not for BelalangBoolean {
    type Output = BelalangBoolean;

    fn not(&self) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(!self.0))
    }
}