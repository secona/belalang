use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, drop_in_place};

use crate::GcObject;

pub struct GcPtr<T: GcObject + ?Sized> {
    ptr: NonNull<T>,
}

impl<T: GcObject + ?Sized> GcPtr<T> {
    pub fn new(ptr: &mut T) -> Self {
        let base = ptr.header();
        base.ref_count.set(base.ref_count.get() + 1);

        // Safety: pointer is valid since we just received it
        Self {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T: GcObject + ?Sized> Deref for GcPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: we maintain the invariant that ptr is always valid when GcPtr is
        // created
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: GcObject + ?Sized> DerefMut for GcPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: we maintain the invariant that ptr is always valid when GcPtr is
        // created
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: GcObject + ?Sized> Debug for GcPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ptr)
    }
}

impl<T: GcObject + ?Sized> Clone for GcPtr<T> {
    fn clone(&self) -> Self {
        let base = self.header();
        base.ref_count.set(base.ref_count.get() + 1);

        Self { ptr: self.ptr }
    }
}

impl<T: GcObject + ?Sized> Drop for GcPtr<T> {
    fn drop(&mut self) {
        let base = self.header();
        base.ref_count.set(base.ref_count.get() - 1);

        if base.ref_count.get() == 0 {
            // Safety: we just checked that ref count is zero
            unsafe { drop_in_place(self.ptr.as_ptr()) };
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, dead_code)]
mod tests {
    use super::*;
    use crate::{GcObjectHeader, with_heap};

    #[derive(Clone)]
    struct Integer {
        header: GcObjectHeader,
        value: i64,
    }

    impl Integer {
        pub fn new(value: i64) -> Integer {
            Self {
                header: GcObjectHeader::new::<Self>(),
                value,
            }
        }
    }

    impl GcObject for Integer {
        fn header(&self) -> &GcObjectHeader {
            &self.header
        }

        fn header_mut(&mut self) -> &mut GcObjectHeader {
            &mut self.header
        }

        fn type_name() -> String
        where
            Self: Sized,
        {
            String::from("Integer")
        }
    }

    #[test]
    fn increments_ref_count() {
        let int = with_heap(|heap| heap.alloc(Integer::new(1)).unwrap());
        assert_eq!(int.header().ref_count.get(), 1);
    }

    #[test]
    fn increments_ref_count_2() {
        let int = with_heap(|heap| heap.alloc(Integer::new(1)).unwrap());
        let int2 = int.clone(); // should increment the ref_count to 2

        assert_eq!(int.header().ref_count.get(), 2);
    }

    #[test]
    fn drop_decrements_ref_count() {
        let int = with_heap(|heap| heap.alloc(Integer::new(1)).unwrap());
        let int2 = int.clone();

        assert_eq!(int.header().ref_count.get(), 2); // from 2

        drop(int2);

        assert_eq!(int.header().ref_count.get(), 1); // to 1
    }
}
