use std::fmt::Display;

use belalang_devel::errors::RuntimeError;
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

    fn add(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        self.0
            .checked_add(other.0)
            .map(Self)
            .map(|v| Box::new(v) as _)
            .ok_or(RuntimeError::IntegerOverflow)
    }

    fn sub(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        self.0
            .checked_sub(other.0)
            .map(Self)
            .map(|v| Box::new(v) as _)
            .ok_or(RuntimeError::IntegerOverflow)
    }

    fn mul(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        self.0
            .checked_mul(other.0)
            .map(Self)
            .map(|v| Box::new(v) as _)
            .ok_or(RuntimeError::IntegerOverflow)
    }

    fn div(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        self.0
            .checked_div(other.0)
            .map(Self)
            .map(|v| Box::new(v) as _)
            .ok_or(RuntimeError::IntegerOverflow)
    }

    fn r#mod(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        self.0
            .checked_rem(other.0)
            .map(Self)
            .map(|v| Box::new(v) as _)
            .ok_or(RuntimeError::IntegerOverflow)
    }

    fn eq(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean(self.0 == other.0)))
    }

    fn ne(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean(self.0 != other.0)))
    }

    fn lt(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean(self.0 < other.0)))
    }

    fn le(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean(self.0 <= other.0)))
    }

    fn bit_and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangInteger(self.0 & other.0)))
    }

    fn bit_or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangInteger(self.0 | other.0)))
    }

    fn bit_xor(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangInteger(self.0 ^ other.0)))
    }

    fn bit_sl(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangInteger(self.0 << other.0)))
    }

    fn bit_sr(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangInteger>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangInteger(self.0 >> other.0)))
    }

    fn neg(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Ok(Box::new(BelalangInteger(-self.0)))
    }
}
