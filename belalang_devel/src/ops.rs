use std::error::Error;

use crate::BelalangType;

pub trait Add<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn add(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Sub<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn sub(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Mul<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn mul(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Div<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn div(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Mod<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn r#mod(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Lt<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn lt(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Le<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn le(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitAnd<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_and(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitOr<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_or(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitXor<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_xor(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitSl<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_sl(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitSr<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_sr(&self, other: &Rhs) -> Result<Self::Output, Box<dyn Error>>;
}
