use std::fmt::Display;

use crate::errors::RuntimeError;
use crate::types::{match_belalang_type, BelalangType};

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
                next: None,
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
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                Ok(Box::new(BelalangBoolean::new(self.value && other.value)) as _)
            },
        )
    }

    fn or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                Ok(Box::new(BelalangBoolean::new(self.value || other.value)) as _)
            },
        )
    }

    fn not(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Ok(Box::new(BelalangBoolean::new(!self.value)))
    }
}
