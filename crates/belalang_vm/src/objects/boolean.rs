use std::fmt::Display;

use belalang_macros::belalang_object;

use crate::core::VM;
use crate::errors::RuntimeError;
use crate::objects::match_belalang_type;
use crate::objects::{BelalangObject, BelalangOperators};
use crate::BelalangBase;

use super::ptr::BelalangPtr;

#[belalang_object(name = "Boolean")]
pub struct BelalangBoolean {
    pub value: bool,
}

impl BelalangBoolean {
    pub fn new(value: bool) -> Self {
        Self {
            base: BelalangBase::new::<Self>(),
            value,
        }
    }
}

impl Display for BelalangBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BelalangOperators for BelalangBoolean {
    fn truthy(&self) -> bool {
        self.value
    }

    fn and(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value && other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }

    fn or(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value || other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }

    fn not(&self, vm: &mut VM) -> Result<BelalangPtr, RuntimeError> {
        let result = BelalangBoolean::new(!self.value);
        let ptr = vm.heap.alloc(result)?;

        Ok(ptr)
    }

    fn eq(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value == other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }

    fn ne(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value != other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }
}
