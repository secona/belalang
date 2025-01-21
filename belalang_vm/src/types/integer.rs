use std::fmt::Display;

use crate::errors::RuntimeError;
use crate::types::boolean::BelalangBoolean;
use crate::types::match_belalang_type;
use crate::types::object::BelalangObject;
use crate::types::BelalangType;

#[repr(C)]
#[derive(Debug)]
pub struct BelalangInteger {
    pub base: BelalangObject,
    pub value: i64,
}

impl BelalangInteger {
    pub fn new(value: i64) -> Self {
        Self {
            base: BelalangObject {
                obj_type: Self::r#type(),
                is_marked: false,
                next: None,
            },
            value,
        }
    }
}

impl Display for BelalangInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BelalangType for BelalangInteger {
    fn type_name() -> String {
        "Integer".into()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn truthy(&self) -> bool {
        self.value != 0
    }

    fn add(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| self.value
                .checked_add(other.value)
                .map(Self::new)
                .map(|v| Box::new(v) as _)
                .ok_or(RuntimeError::IntegerOverflow)
        )
    }

    fn sub(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| self.value
                .checked_sub(other.value)
                .map(Self::new)
                .map(|v| Box::new(v) as _)
                .ok_or(RuntimeError::IntegerOverflow)
        )
    }

    fn mul(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| self.value
                .checked_mul(other.value)
                .map(Self::new)
                .map(|v| Box::new(v) as _)
                .ok_or(RuntimeError::IntegerOverflow)
        )
    }

    fn div(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| self.value
                .checked_div(other.value)
                .map(Self::new)
                .map(|v| Box::new(v) as _)
                .ok_or(RuntimeError::IntegerOverflow)
        )
    }

    fn r#mod(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| self.value
                .checked_rem(other.value)
                .map(Self::new)
                .map(|v| Box::new(v) as _)
                .ok_or(RuntimeError::IntegerOverflow)
        )
    }

    fn eq(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangBoolean::new(self.value == other.value)) as _)
            }
        )
    }

    fn ne(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangBoolean::new(self.value != other.value)) as _)
            }
        )
    }

    fn lt(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangBoolean::new(self.value < other.value)) as _)
            }
        )
    }

    fn le(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangBoolean::new(self.value <= other.value)) as _)
            }
        )
    }

    fn bit_and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangInteger::new(self.value & other.value)) as _)
            }
        )
    }

    fn bit_or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangInteger::new(self.value | other.value)) as _)
            }
        )
    }

    fn bit_xor(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangInteger::new(self.value ^ other.value)) as _)
            }
        )
    }

    fn bit_sl(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangInteger::new(self.value << other.value)) as _)
            }
        )
    }

    fn bit_sr(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                Ok(Box::new(BelalangInteger::new(self.value >> other.value)) as _)
            }
        )
    }

    fn neg(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Ok(Box::new(BelalangInteger::new(-self.value)))
    }
}
