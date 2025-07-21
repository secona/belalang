use std::fmt::Display;

use crate::BelalangBase;
use crate::BelalangPtr;
use crate::errors::RuntimeError;
use crate::objects::boolean::BelalangBoolean;
use crate::objects::match_belalang_type;
use crate::objects::{BelalangObject, BelalangOperators};
use crate::with_heap;

#[repr(C)]
#[derive(Debug)]
pub struct BelalangInteger {
    pub base: BelalangBase,
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

impl BelalangObject for BelalangInteger {
    fn type_name() -> String {
        "Integer".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn base(&self) -> &BelalangBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BelalangBase {
        &mut self.base
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

    fn add(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_add(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn sub(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_sub(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn mul(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_mul(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn div(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_div(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn r#mod(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let Some(result) = self.value.checked_rem(other.value) else {
                    return Err(RuntimeError::TypeError);
                };

                let result = Self::new(result);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn eq(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value == other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn ne(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value != other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn lt(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value < other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn le(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangBoolean::new(self.value <= other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn bit_and(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value & other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn bit_or(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value | other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn bit_xor(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value ^ other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn bit_sl(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value << other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn bit_sr(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let result = BelalangInteger::new(self.value >> other.value);
                let ptr = with_heap(|heap| heap.alloc(result))?;

                Ok(ptr)
            }
        )
    }

    fn neg(&self) -> Result<BelalangPtr, RuntimeError> {
        let result = BelalangInteger::new(-self.value);
        let ptr = with_heap(|heap| heap.alloc(result))?;

        Ok(ptr)
    }
}
