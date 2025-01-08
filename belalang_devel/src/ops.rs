use std::error::Error;

use crate::BelalangType;

pub trait Add: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn add(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Sub: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn sub(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Mul: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn mul(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Div: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn div(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Mod: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn r#mod(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Eq: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn eq(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Ne: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn ne(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Lt: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn lt(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Le: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn le(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait And: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn and(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait Or: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn or(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitAnd: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn bit_and(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitOr: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn bit_or(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitXor: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn bit_xor(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitSl: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn bit_sl(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}

pub trait BitSr: BelalangType {
    type Output: BelalangType;
    type Rhs: BelalangType;
    fn bit_sr(&self, other: &Self::Rhs) -> Result<Self::Output, Box<dyn Error>>;
}
