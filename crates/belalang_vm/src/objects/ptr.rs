use std::ptr::{drop_in_place, NonNull};

use crate::BelalangBase;

use super::BelalangObject;

pub struct BelalangPtr {
    ptr: NonNull<dyn BelalangObject>,
}

impl BelalangPtr {
    pub fn new(ptr: NonNull<dyn BelalangObject>) -> Self {
        unsafe {
            let base_ptr = ptr.as_ptr() as *mut BelalangBase;
            (*base_ptr).ref_count.set((*base_ptr).ref_count.get() + 1);
        };

        Self { ptr }
    }
}

impl Drop for BelalangPtr {
    fn drop(&mut self) {
        let inner = unsafe {
            let ptr = self.ptr.as_ptr() as *const BelalangBase;
            ptr.read()
        };

        let ref_count = inner.ref_count.get();

        if ref_count == 1 {
            unsafe { drop_in_place(self.ptr.as_ptr()) };
        } else {
            inner.ref_count.set(ref_count - 1);
        }
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
        let ptr = BelalangPtr::new(int);

        let base = unsafe {
            let ptr = int.as_ptr() as *const BelalangBase;
            ptr.read()
        };

        assert_eq!(base.ref_count.get(), 1);
    }

    #[test]
    fn increments_ref_count_2() {
        let mut heap = Heap::default();

        let int = heap.alloc(BelalangInteger::new(1)).unwrap();
        let ptr1 = BelalangPtr::new(int);
        let ptr2 = BelalangPtr::new(int);

        let base = unsafe {
            let ptr = int.as_ptr() as *const BelalangBase;
            ptr.read()
        };

        assert_eq!(base.ref_count.get(), 2);
    }
}
