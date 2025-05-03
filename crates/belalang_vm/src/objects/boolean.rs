use std::fmt::Display;

use crate::BelalangBase;
use crate::core::BelalangPtr;
use crate::core::VM;
use crate::errors::RuntimeError;
use crate::objects::match_belalang_type;
use crate::objects::{BelalangObject, BelalangOperators};

#[repr(C)]
#[derive(Debug)]
pub struct BelalangBoolean {
    pub base: BelalangBase,
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

impl BelalangObject for BelalangBoolean {
    fn type_name() -> String {
        "Boolean".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
