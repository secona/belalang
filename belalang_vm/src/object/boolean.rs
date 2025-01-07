#![allow(unused_variables)]

use std::fmt::Display;

use super::ObjTrait;

#[derive(Debug)]
pub struct BelalangBoolean(bool);

impl Display for BelalangBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ObjTrait for BelalangBoolean {
    fn try_add(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_sub(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_mul(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_div(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_mod(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_less_than(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_less_than_equal(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_bit_and(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        Ok(Self(self.0 & rhs.0))
    }

    fn try_bit_or(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        Ok(Self(self.0 | rhs.0))
    }

    fn try_bit_xor(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        Ok(Self(self.0 ^ rhs.0))
    }

    fn try_bit_sl(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }

    fn try_bit_sr(self, rhs: Self) -> Result<Self, crate::error::RuntimeError> {
        todo!()
    }
}
