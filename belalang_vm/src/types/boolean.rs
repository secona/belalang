use std::fmt::Display;
use std::ptr::NonNull;

use crate::errors::RuntimeError;
use crate::types::match_belalang_type;
use crate::types::object::BelalangObject;
use crate::types::BelalangType;
use crate::vm::VM;

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

    fn and(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value && other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }

    fn or(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value || other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }

    fn not(&self, vm: &mut VM) -> Result<NonNull<BelalangObject>, RuntimeError> {
        let result = BelalangBoolean::new(!self.value);
        let ptr = vm.heap.alloc(result)?;

        Ok(ptr)
    }

    fn eq(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value == other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }

    fn ne(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangBoolean => |other: &BelalangBoolean| {
                let result = BelalangBoolean::new(self.value != other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            },
        )
    }
}
