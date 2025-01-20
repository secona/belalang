use std::fmt::Display;

use crate::{errors::RuntimeError, types::BelalangType};

use super::BelalangObject;

#[repr(C)]
#[derive(Debug)]
pub struct BelalangBoolean {
    pub base: BelalangObject,
    pub value: bool,
}

impl BelalangBoolean {
    pub fn new(value: bool) -> Self {
        Self {
            base: BelalangObject {
                obj_type: Self::r#type(),
                is_marked: false,
                next: std::ptr::null_mut(),
            },
            value,
        }
    }
}

impl Display for BelalangBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BelalangType for BelalangBoolean {
    fn type_name() -> String {
        "Boolean".into()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn truthy(&self) -> bool {
        self.value
    }

    fn and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangBoolean>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean::new(self.value && other.value)))
    }

    fn or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        let Some(other) = other.as_any().downcast_ref::<BelalangBoolean>() else {
            return Err(RuntimeError::TypeError);
        };

        Ok(Box::new(BelalangBoolean::new(self.value || other.value)))
    }

    fn not(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Ok(Box::new(BelalangBoolean::new(!self.value)))
    }
}
