use std::any::Any;
use std::fmt::{Debug, Display};
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

use crate::errors::RuntimeError;

pub mod boolean;
pub mod integer;

#[repr(C)]
#[derive(Debug)]
pub struct BelalangObject {
    pub obj_type: u32,
    pub is_marked: bool,
    pub next: Option<NonNull<BelalangObject>>,
}

impl PartialEq for BelalangObject {
    fn eq(&self, other: &Self) -> bool {
        self.obj_type == other.obj_type
    }
}

#[allow(unused_variables)]
pub trait BelalangType: Display + Debug {
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

    fn truthy(&self) -> bool {
        false
    }

    // NOTE: I am thinking of changing these methods to take in pointers
    // to BelalangObject and returning pointers to BelalangObject.

    fn add(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn sub(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn mul(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn div(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn r#mod(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn eq(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn ne(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn lt(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn le(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_and(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_or(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_xor(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sl(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn bit_sr(&self, other: &dyn BelalangType) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn neg(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }

    fn not(&self) -> Result<Box<dyn BelalangType>, RuntimeError> {
        Err(RuntimeError::TypeError)
    }
}

impl PartialEq for dyn BelalangType {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Fix this!
        format!("{}", self) == format!("{}", other)
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
