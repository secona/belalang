use std::fmt::Display;

use belalang_macros::belalang_object;

use crate::core::VM;
use crate::errors::RuntimeError;
use crate::objects::boolean::BelalangBoolean;
use crate::objects::match_belalang_type;
use crate::objects::{BelalangObject, BelalangOperators};
use crate::BelalangBase;

use super::ptr::BelalangPtr;

#[belalang_object(name = "Integer")]
pub struct BelalangInteger {
    pub value: i64,
}

impl BelalangInteger {
    pub fn new(value: i64) -> Self {
        Self {
            base: BelalangBase::new::<Self>(),
            value,
        }
    }
}

impl Display for BelalangInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BelalangOperators for BelalangInteger {
    fn truthy(&self) -> bool {
        self.value != 0
    }

    fn add(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
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

    fn sub(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
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

    fn mul(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
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

    fn div(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
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

    fn r#mod(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
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

    fn eq(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value == other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn ne(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value != other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn lt(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value < other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn le(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value <= other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_and(
        &self,
        vm: &mut VM,
        other: &dyn BelalangObject,
    ) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value & other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_or(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value | other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_xor(
        &self,
        vm: &mut VM,
        other: &dyn BelalangObject,
    ) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value ^ other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_sl(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value << other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn bit_sr(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value >> other.value);
                let ptr = vm.heap.alloc(result)?;

                Ok(ptr)
            }
        )
    }

    fn neg(&self, vm: &mut VM) -> Result<BelalangPtr, RuntimeError> {
        let result = BelalangInteger::new(-self.value);
        let ptr = vm.heap.alloc(result)?;

        Ok(ptr)
    }
}
