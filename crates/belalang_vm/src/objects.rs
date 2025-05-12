//! Builtin objects provided by The Belalang Virtual Machine.
//!
//! # Note
//! This structure may change. See [`crate::builtins`]

use std::any::Any;
use std::cell::Cell;
use std::fmt::{Debug, Display};
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

use crate::core::BelalangPtr;
use crate::core::VM;
use crate::errors::RuntimeError;

pub mod array;
pub mod boolean;
pub mod integer;

#[derive(Debug)]
#[repr(C)]
pub struct BelalangBase {
    pub obj_type: u32,
    pub ref_count: Cell<usize>,
    pub is_marked: bool,
    pub next: Option<NonNull<BelalangBase>>,
}

impl BelalangBase {
    pub fn new<T: BelalangObject>() -> Self {
        Self {
            obj_type: T::r#type(),
            ref_count: Cell::new(0),
            is_marked: false,
            next: None,
        }
    }
}

pub trait BelalangObject: BelalangOperators + Display + Debug {
    fn type_name() -> String
    where
        Self: Sized;

    fn r#type() -> u32
    where
        Self: Sized,
    {
        let mut hasher = DefaultHasher::new();
        Self::type_name().hash(&mut hasher);
        let hash = hasher.finish();
        hash as u32
    }

    fn as_any(&self) -> &dyn Any;
}

#[allow(unused_variables)]
pub trait BelalangOperators {
    fn truthy(&self) -> bool {
        false
    }

    fn add(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn sub(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn mul(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn div(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn r#mod(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn eq(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn ne(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn lt(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn le(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn and(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn or(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_and(
        &self,
        vm: &mut VM,
        other: &dyn BelalangObject,
    ) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_or(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_xor(
        &self,
        vm: &mut VM,
        other: &dyn BelalangObject,
    ) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sl(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sr(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn neg(&self, vm: &mut VM) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn not(&self, vm: &mut VM) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }
}

macro_rules! match_belalang_type {
    ($other:expr, $($type:ty => $body:expr),* $(,)?) => {
        match $other.as_any() {
            $(
                x if x.is::<$type>() => {
                    let other = $other
                        .as_any()
                        .downcast_ref::<$type>()
                        .expect("Type check succeeded but downcast failed");
                    $body(other)
                },
            )*
            _ => Err(RuntimeError::TypeError),
        }
    };

    ($other:expr, $($type:ty => $body:expr),*) => {
        match_belalang_type!($other, $($type => $body),*,)
    };
}

pub(crate) use match_belalang_type;
