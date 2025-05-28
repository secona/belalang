use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, drop_in_place};

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
    /// [`ref_count`][crate::BelalangBase::ref_count] by one (this pointer itself).
    pub fn new(ptr: NonNull<dyn BelalangObject>) -> Self {
        let ptr = Self { ptr };

        let base = ptr.base();
        base.ref_count.set(base.ref_count.get() + 1);

        ptr
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
        let base = self.base();
        base.ref_count.set(base.ref_count.get() + 1);

        Self { ptr: self.ptr }
    }
}

impl Deref for BelalangPtr {
    type Target = dyn BelalangObject;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl DerefMut for BelalangPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl Drop for BelalangPtr {
    fn drop(&mut self) {
        let base = self.base();
        base.ref_count.set(base.ref_count.get() - 1);

        if base.ref_count.get() == 0 {
            unsafe { drop_in_place(self.ptr.as_ptr()) };
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use crate::mem::heap::Heap;
    use crate::objects::integer::BelalangInteger;

    #[test]
    fn increments_ref_count() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();

        assert_eq!(int.base().ref_count.get(), 1);
    }

    #[test]
    fn increments_ref_count_2() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();
        let int2 = int.clone(); // should increment the ref_count to 2

        assert_eq!(int.base().ref_count.get(), 2);
    }

    #[test]
    fn drop_decrements_ref_count() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();
        let int2 = int.clone();

        assert_eq!(int.base().ref_count.get(), 2); // from 2

        drop(int2);

        assert_eq!(int.base().ref_count.get(), 1); // to 1
    }
}
