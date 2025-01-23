use std::alloc::{alloc, Layout};
use std::fmt::Display;

use belalang_macros::belalang_type;

use crate::types::object::BelalangObject;
use crate::types::BelalangType;

#[belalang_type]
pub struct BelalangString {
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
            base: BelalangObject::new::<Self>(),
            ptr,
            len,
            cap,
        }
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

impl BelalangType for BelalangString {
    fn type_name() -> String
    where
        Self: Sized
    {
        "String".into()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
