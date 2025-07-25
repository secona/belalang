use std::alloc::{Layout, alloc, dealloc};
use std::fmt::Display;
use std::ptr::drop_in_place;

use super::BelalangObject;
use crate::objects::BelalangOperators;
use crate::{BelalangBase, BelalangPtr};

#[repr(C)]
#[derive(Debug)]
pub struct BelalangArray {
    pub base: BelalangBase,
    pub ptr: *mut BelalangPtr,
    pub len: usize,
    pub cap: usize,
}

impl Drop for BelalangArray {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                let elem_ptr = self.ptr.add(i);
                drop_in_place(elem_ptr);
            }

            if !self.ptr.is_null() && self.cap > 0 {
                let layout = Layout::from_size_align(self.cap, align_of::<BelalangPtr>()).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

impl BelalangObject for BelalangArray {
    fn type_name() -> String {
        "Array".to_string()
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

impl Display for BelalangArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Array")
    }
}

impl BelalangArray {
    // pub fn new(string: &'static str) -> Self {
    //     let len = string.len();
    //     let cap = string.len();
    //
    //     let ptr = unsafe {
    //         let layout =
    //             Layout::from_size_align(len, align_of::<*mut dyn
    // BelalangObject>()).unwrap();         let ptr = alloc(layout);
    //
    //         if ptr.is_null() {
    //             panic!("Failed to allocate memory for BelalangArray");
    //         }
    //
    //         ptr as *mut *mut dyn BelalangObject
    //     };
    //
    //     Self {
    //         base: BelalangBase::new::<Self>(),
    //         ptr,
    //         len,
    //         cap,
    //     }
    // }

    pub fn with_capacity(cap: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align(cap, align_of::<BelalangPtr>()).unwrap();
            let ptr = alloc(layout);

            if ptr.is_null() {
                panic!("Failed to allocate memory for BelalangArray");
            }

            ptr as *mut BelalangPtr
        };

        Self {
            base: BelalangBase::new::<Self>(),
            ptr,
            len: cap,
            cap,
        }
    }
}

impl BelalangOperators for BelalangArray {}
