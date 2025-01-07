#![allow(unused_variables)]

use std::{error::Error, fmt::Display};

use crate::error::RuntimeError;

use belalang_devel::ops::Add;
use belalang_devel::BelalangType;

#[derive(Debug, Clone)]
pub struct BelalangInteger(pub i64);

impl Display for BelalangInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BelalangType for BelalangInteger {
    fn type_name(&self) -> &str {
        "Integer"
    }
}

impl Add for BelalangInteger {
    type Output = BelalangInteger;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn add(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(Box::new(RuntimeError::IntegerOverflow))
    }
}
