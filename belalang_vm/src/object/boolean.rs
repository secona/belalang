#![allow(unused_variables)]

use std::{error::Error, fmt::Display};

use belalang_devel::BelalangType;
use belalang_devel::ops::{And, Or};

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

impl And<BelalangBoolean> for BelalangBoolean {
    type Output = BelalangBoolean;

    fn and(&self, other: &BelalangBoolean) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 && other.0))
    }
}

impl Or<BelalangBoolean> for BelalangBoolean {
    type Output = BelalangBoolean;

    fn and(&self, other: &BelalangBoolean) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 || other.0))
    }
}
