use std::alloc::{alloc, dealloc, Layout};
use std::fmt::Display;

use belalang_macros::belalang_object;

use crate::objects::{BelalangOperators, BelalangObject};
use crate::BelalangBase;

#[belalang_object(name = "Array")]
pub struct BelalangArray {
    pub ptr: *mut *mut dyn BelalangObject,
    pub len: usize,
    pub cap: usize,
}

impl Drop for BelalangArray {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let layout = Layout::new::<Self>();
            unsafe { dealloc(self.ptr as *mut u8, layout) };
        }
    }
}

impl Display for BelalangArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Array")
    }
}

impl BelalangArray {
    pub fn new(string: &'static str) -> Self {
        let len = string.len();
        let cap = string.len();

        let ptr = unsafe {
            let layout = Layout::from_size_align(len, align_of::<*mut dyn BelalangObject>()).unwrap();
            let ptr = alloc(layout);

            if ptr.is_null() {
                panic!("Failed to allocate memory for BelalangArray");
            }

            ptr as *mut *mut dyn BelalangObject
        };

        Self {
            base: BelalangBase::new::<Self>(),
            ptr,
            len,
            cap,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align(cap, align_of::<*mut dyn BelalangObject>()).unwrap();
            let ptr = alloc(layout);

            if ptr.is_null() {
                panic!("Failed to allocate memory for BelalangArray");
            }

            ptr as *mut *mut dyn BelalangObject
        };

        Self {
            base: BelalangBase::new::<Self>(),
            ptr,
            len: 0,
            cap,
        }
    }
}

impl BelalangOperators for BelalangArray {}
