use std::fmt::Display;

use crate::errors::RuntimeError;
use crate::types::BelalangType;

#[derive(Debug, Clone)]
pub struct BelalangBoolean(pub bool);

impl Display for BelalangBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BelalangType for BelalangBoolean {
    fn type_name(&self) -> &str {
        "Boolean"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn truthy(&self) -> bool {
        self.0
    }

    fn and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangBoolean>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean(self.0 && other.0)))
    }

    fn or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangBoolean>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean(self.0 || other.0)))
    }

    fn not(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Ok(Box::new(BelalangBoolean(!self.0)))
    }
}
