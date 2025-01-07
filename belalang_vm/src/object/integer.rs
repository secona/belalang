#![allow(unused_variables)]

use std::fmt::Display;

use crate::error::RuntimeError;

use super::{Add, BelalangType};

#[derive(Debug)]
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

impl Add<BelalangInteger> for BelalangInteger {
    type Output = BelalangInteger;

    fn add(&self, other: &BelalangInteger) -> Result<Self::Output, RuntimeError> {
        self.0.checked_add(other.0)
            .map(Self)
            .ok_or(RuntimeError::IntegerOverflow)
    }
}
