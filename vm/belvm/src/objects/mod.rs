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

use crate::BelalangPtr;
use crate::errors::RuntimeError;

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
    fn base(&self) -> &BelalangBase;
    fn base_mut(&mut self) -> &mut BelalangBase;

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

    fn add(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn sub(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn mul(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn div(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn r#mod(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn eq(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn ne(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn lt(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn le(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn and(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn or(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_and(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_or(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_xor(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sl(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sr(&self, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn neg(&self) -> Result<BelalangPtr, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn not(&self) -> Result<BelalangPtr, RuntimeError> {
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
