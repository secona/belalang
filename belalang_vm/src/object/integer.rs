#![allow(unused_variables)]

use std::error::Error;
use std::fmt::Display;

use crate::error::RuntimeError;

use belalang_devel::ops::{Add, BitAnd, BitOr, BitSl, BitSr, BitXor, Div, Eq, Le, Lt, Mod, Mul, Ne, Neg, Sub};
use belalang_devel::BelalangType;

use super::boolean::BelalangBoolean;

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

impl Sub for BelalangInteger {
    type Output = BelalangInteger;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn sub(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(Box::new(RuntimeError::IntegerOverflow))
    }
}

impl Mul for BelalangInteger {
    type Output = BelalangInteger;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn mul(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        self.0
            .checked_mul(other.0)
            .map(Self)
            .ok_or(Box::new(RuntimeError::IntegerOverflow))
    }
}

impl Div for BelalangInteger {
    type Output = BelalangInteger;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn div(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        self.0
            .checked_div(other.0)
            .map(Self)
            .ok_or(Box::new(RuntimeError::IntegerOverflow))
    }
}

impl Mod for BelalangInteger {
    type Output = BelalangInteger;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn r#mod(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        Ok(Self(self.0 % other.0))
    }
}

impl Eq for BelalangInteger {
    type Output = BelalangBoolean;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn eq(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 == other.0))
    }
}

impl Ne for BelalangInteger {
    type Output = BelalangBoolean;

    // This is a temporary fix. It should be replaced with BelalangType.
    type Rhs = BelalangInteger;

    fn ne(&self, other: &BelalangInteger) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 != other.0))
    }
}

impl Lt for BelalangInteger {
    type Output = BelalangBoolean;

    type Rhs = BelalangInteger;

    fn lt(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 < other.0))
    }
}

impl Le for BelalangInteger {
    type Output = BelalangBoolean;

    type Rhs = BelalangInteger;

    fn le(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangBoolean(self.0 <= other.0))
    }
}

impl BitAnd for BelalangInteger {
    type Output = BelalangInteger;

    type Rhs = BelalangInteger;

    fn bit_and(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangInteger(self.0 & other.0))
    }
}

impl BitOr for BelalangInteger {
    type Output = BelalangInteger;

    type Rhs = BelalangInteger;

    fn bit_or(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangInteger(self.0 | other.0))
    }
}

impl BitXor for BelalangInteger {
    type Output = BelalangInteger;

    type Rhs = BelalangInteger;

    fn bit_xor(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangInteger(self.0 ^ other.0))
    }
}

impl BitSl for BelalangInteger {
    type Output = BelalangInteger;

    type Rhs = BelalangInteger;

    fn bit_sl(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangInteger(self.0 << other.0))
    }
}

impl BitSr for BelalangInteger {
    type Output = BelalangInteger;

    type Rhs = BelalangInteger;

    fn bit_sr(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangInteger(self.0 >> other.0))
    }
}

impl Neg for BelalangInteger {
    type Output = BelalangInteger;

    fn neg(&self) -> Result<Self::Output, Box<dyn Error>> {
        Ok(BelalangInteger(-self.0))
    }
}
