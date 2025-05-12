use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::ptr::{drop_in_place, NonNull};

use crate::GcObject;

pub struct GcPtr {
    ptr: NonNull<dyn GcObject>,
}

impl GcPtr {
    pub fn new(ptr: NonNull<dyn GcObject>) -> Self {
        let ptr = Self { ptr };

        let base = ptr.header();
        base.ref_count.set(base.ref_count.get() + 1);

        ptr
    }

    pub fn as_ptr(&self) -> *mut dyn GcObject {
        self.ptr.as_ptr()
    }
}

impl Deref for GcPtr {
    type Target = dyn GcObject;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl DerefMut for GcPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl Debug for GcPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ptr)
    }
}

impl Clone for GcPtr {
    fn clone(&self) -> Self {
        let base = self.header();
        base.ref_count.set(base.ref_count.get() + 1);

        Self { ptr: self.ptr }
    }
}

impl Drop for GcPtr {
    fn drop(&mut self) {
        let base = self.header();
        base.ref_count.set(base.ref_count.get() - 1);

        if base.ref_count.get() == 0 {
            unsafe { drop_in_place(self.ptr.as_ptr()) };
        }
    }
}
