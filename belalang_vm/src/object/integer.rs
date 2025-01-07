#![allow(unused_variables)]

use std::{error::Error, fmt::Display};

use crate::error::RuntimeError;

use belalang_devel::BelalangType;
use belalang_devel::ops::Add;

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

    fn add(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        self.0.checked_add(other.0)
            .map(Self)
            .ok_or(Box::new(RuntimeError::IntegerOverflow))
    }
}
