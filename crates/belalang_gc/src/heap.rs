use std::hash::{DefaultHasher, Hash, Hasher};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::cell::Cell;
use std::alloc::{alloc, Layout};

use crate::{GcPtr, MemoryError};

#[derive(Clone)]
pub struct GcObjectHeader {
    pub obj_type: u32,
    pub ref_count: Cell<usize>,
    pub is_marked: bool,
    pub next: Option<NonNull<dyn GcObject>>,
}

impl GcObjectHeader {
    pub fn new<T: GcObject>() -> Self {
        Self {
            obj_type: T::r#type(),
            ref_count: Cell::new(0),
            is_marked: false,
            next: None,
        }
    }
}

pub trait GcObject {
    fn header(&self) -> &GcObjectHeader;
    fn header_mut(&mut self) -> &mut GcObjectHeader;

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
}

#[derive(Default)]
pub struct GcHeap {
    pub start: Option<NonNull<dyn GcObject>>,
    _marker: PhantomData<GcObjectHeader>,
}

impl GcHeap {
    pub fn alloc<T: GcObject>(&mut self, object: T) -> Result<GcPtr<T>, MemoryError> {
        let layout = Layout::new::<T>();

        let base_ptr: *mut T = unsafe {
            let ptr = alloc(layout) as *mut T;

            if ptr.is_null() {
                return Err(MemoryError::AllocationFailed);
            }

            ptr.write(object);

            ptr
        };

        // Safety: base_ptr was just created in this function call
        let ptr = base_ptr as *mut dyn GcObject;
        unsafe { (*ptr).header_mut().next = self.start };

        // Safety: base_ptr was just created in this function call
        unsafe { self.start = Some(NonNull::new_unchecked(ptr)) };

        // Safety: base_ptr was just created in this function call
        unsafe { Ok(GcPtr::new(&mut *base_ptr)) }
    }
}

impl Drop for GcHeap {
    fn drop(&mut self) {
        self.start = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                Self: Sized
        {
            String::from("Integer")
        }
    }

    #[test]
    fn test_heap_allocations_1() {
        let values = [
            Integer::new(1),
            Integer::new(2),
            Integer::new(3),
        ];

        let mut heap = GcHeap::default();
        let mut ptrs = Vec::new();

        // Allocate all values
        for value in values.iter().cloned() {
            let ptr = heap.alloc(value);
            ptrs.push(ptr);
        }

        // Check structure
        let mut current = heap.start;
        for (i, expected) in values.iter().rev().enumerate() {
            let Some(ptr) = current else {
                panic!("Heap has fewer elements than expected at position {}", i);
            };

            let int = unsafe { &*(ptr.as_ptr() as *const Integer) };
            assert_eq!(int.header().obj_type, Integer::r#type());
            assert_eq!(int.value, expected.value);

            current = int.header().next;
        }

        // Ensure we've reached the end of the heap
        assert!(current.is_none(), "Heap has more elements than expected");
    }

    // TODO: allocate multiple types
}
