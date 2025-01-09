use std::any::Any;
use std::fmt::{Debug, Display};

use errors::RuntimeError;

pub mod errors;

// I allow unused_variables to prevent the `other` variable
// of needing an underscore at the start.
#[allow(unused_variables)]
pub trait BelalangType: Display + Debug {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;

    fn equal_type(&self, other: &dyn BelalangType) -> bool {
        self.type_name() == other.type_name()
    }

    fn add(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn sub(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn mul(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn div(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn r#mod(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn eq(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn ne(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn lt(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn le(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_xor(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sl(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sr(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn neg(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn not(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }
}

impl PartialEq for dyn BelalangType {
    fn eq(&self, other: &Self) -> bool {
        // Temporary implementation
        self.equal_type(other) && format!("{}", self) == format!("{}", other)
    }
}
