use std::alloc::{Layout, alloc};
use std::fmt::Display;

use crate::BelalangBase;
use crate::core::BelalangPtr;
use crate::core::VM;
use crate::errors::RuntimeError;
use crate::objects::integer::BelalangInteger;
use crate::objects::{BelalangObject, BelalangOperators, match_belalang_type};

#[repr(C)]
#[derive(Debug)]
pub struct BelalangString {
    pub base: BelalangBase,
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl BelalangString {
    pub fn new(string: &'static str) -> Self {
        let len = string.len();
        let cap = string.len();

        let ptr = unsafe {
            let layout = Layout::from_size_align(len, align_of::<u8>()).unwrap();
            let ptr = alloc(layout);

            if ptr.is_null() {
                panic!("Failed to allocate memory for BelalangString");
            }

            std::ptr::copy_nonoverlapping(string.as_ptr(), ptr, len);

            ptr
        };

        Self {
            base: BelalangBase::new::<Self>(),
            ptr,
            len,
            cap,
        }
    }
}

impl BelalangObject for BelalangString {
    fn type_name() -> String {
        "String".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for BelalangString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.len);
            std::str::from_utf8(slice).expect("Invalid UTF-8")
        })
    }
}

impl BelalangOperators for BelalangString {
    fn add(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangString => |other: &BelalangString| {
                let len = self.len + other.len;
                let cap = len;

                let ptr = unsafe {
                    let layout = Layout::from_size_align(len, align_of::<u8>()).unwrap();
                    let ptr = alloc(layout);

                    if ptr.is_null() {
                        panic!("Failed to allocate memory for BelalangString");
                    }

                    std::ptr::copy_nonoverlapping(self.ptr, ptr, self.len);
                    std::ptr::copy_nonoverlapping(other.ptr, ptr.add(self.len), other.len);

                    ptr
                };

                vm.heap.alloc(Self {
                    base: BelalangBase::new::<Self>(),
                    ptr,
                    len,
                    cap,
                })
            }
        )
    }

    fn mul(&self, vm: &mut VM, other: &dyn BelalangObject) -> Result<BelalangPtr, RuntimeError> {
        match_belalang_type!(other,
            BelalangInteger => |other: &BelalangInteger| {
                let value = other.value.max(0) as usize;
                let len = self.len * value;
                let cap = self.cap * value;

                let ptr = unsafe {
                    let layout = Layout::from_size_align(len, align_of::<u8>()).unwrap();
                    let ptr = alloc(layout);

                    if ptr.is_null() {
                        panic!("Failed to allocate memory for BelalangString");
                    }

                    for i in 0..value {
                        std::ptr::copy_nonoverlapping(self.ptr, ptr.add(i * self.len), self.len);
                    }

                    ptr
                };

                vm.heap.alloc(Self {
                    base: BelalangBase::new::<Self>(),
                    ptr,
                    len,
                    cap,
                })
            }
        )
    }
}
