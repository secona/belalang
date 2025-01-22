use std::fmt::Display;
use std::ptr::NonNull;

use crate::errors::RuntimeError;
use crate::types::boolean::BelalangBoolean;
use crate::types::match_belalang_type;
use crate::types::object::BelalangObject;
use crate::types::BelalangType;
use crate::vm::VM;

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

    fn add(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_add(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn sub(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_sub(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn mul(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_mul(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn div(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_div(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn r#mod(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_rem(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn eq(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value == other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn ne(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value != other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn lt(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value < other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn le(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value <= other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_and(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value & other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_or(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value | other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_xor(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value ^ other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_sl(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value << other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_sr(&self, vm: &mut VM, other: &dyn BelalangType) -> Result<NonNull<BelalangObject>, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value >> other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn neg(&self, vm: &mut VM) -> Result<NonNull<BelalangObject>, RuntimeError> {
        let result = BelalangInteger::new(-self.value);
        let ptr = vm.heap.alloc(result)?;

        Ok(ptr)
    }
}
