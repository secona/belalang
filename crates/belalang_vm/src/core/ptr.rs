use std::fmt::Debug;
use std::ptr::{NonNull, drop_in_place};

use crate::BelalangBase;

use crate::objects::BelalangObject;

/// Pointer to a BelalangObject
///
/// Provides an interface to storing a BelalangObject through a pointer.
pub struct BelalangPtr {
    ptr: NonNull<dyn BelalangObject>,
}

impl BelalangPtr {
    /// Creates a new [`BelalangPtr`]
    ///
    /// Also does initialization for reference counting by incrementing the
    /// [`ref_count`][BelalangBase::ref_count] by one (this pointer itself).
    pub fn new(ptr: NonNull<dyn BelalangObject>) -> Self {
        unsafe {
            let base_ptr = ptr.as_ptr() as *mut BelalangBase;
            (*base_ptr).ref_count.set((*base_ptr).ref_count.get() + 1);
        };

        Self { ptr }
    }

    /// Raw Rust pointer of [`BelalangPtr`]
    pub fn as_ptr(&self) -> *mut dyn BelalangObject {
        self.ptr.as_ptr()
    }
}

impl Debug for BelalangPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ptr)
    }
}

impl Clone for BelalangPtr {
    fn clone(&self) -> Self {
        unsafe {
            let base_ptr = self.ptr.as_ptr() as *mut BelalangBase;
            (*base_ptr).ref_count.set((*base_ptr).ref_count.get() + 1);
        };

        Self { ptr: self.ptr }
    }
}

impl Drop for BelalangPtr {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self.ptr.as_ptr() as *mut BelalangBase;
            let ref_count = (*base_ptr).ref_count.get();

            let new_count = ref_count - 1;
            (*base_ptr).ref_count.set(new_count);

            if new_count == 0 {
                drop_in_place(self.ptr.as_ptr());
            }
        };
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use crate::mem::heap::Heap;
    use crate::objects::integer::BelalangInteger;

    use super::*;

    #[test]
    fn increments_ref_count() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();

        let ref_count = unsafe {
            let base_ptr = int.as_ptr() as *const BelalangBase;
            (*base_ptr).ref_count.get()
        };
        assert_eq!(ref_count, 1);
    }

    #[test]
    fn increments_ref_count_2() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();
        let int2 = int.clone();

        let ref_count = unsafe {
            let base_ptr = int.as_ptr() as *const BelalangBase;
            (*base_ptr).ref_count.get()
        };
        assert_eq!(ref_count, 2);
    }

    #[test]
    fn drop_decrements_ref_count() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();
        let int2 = int.clone();

        let ref_count = unsafe {
            let base_ptr = int.as_ptr() as *const BelalangBase;
            (*base_ptr).ref_count.get()
        };
        assert_eq!(ref_count, 2);

        drop(int2);

        let ref_count = unsafe {
            let base_ptr = int.as_ptr() as *const BelalangBase;
            (*base_ptr).ref_count.get()
        };
        assert_eq!(ref_count, 1);
    }
}
