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

    #[derive(Clone)]
    struct Float {
        header: GcObjectHeader,
        value: f64,
    }

    impl Float {
        pub fn new(value: f64) -> Float {
            Self {
                header: GcObjectHeader::new::<Self>(),
                value,
            }
        }
    }

    impl GcObject for Float {
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
    fn test_heap_allocations() {
        let mut heap = GcHeap::default();

        heap.alloc(Integer::new(1)).unwrap();
        heap.alloc(Float::new(2.0)).unwrap();
        heap.alloc(Integer::new(3)).unwrap();

        let current = heap.start.unwrap();

        let c = unsafe { &*(current.as_ptr() as *const Integer) };
        assert_eq!(c.header().obj_type, Integer::r#type());
        assert_eq!(c.value, 3);
        let current = c.header().next.unwrap();

        let c = unsafe { &*(current.as_ptr() as *const Float) };
        assert_eq!(c.header().obj_type, Float::r#type());
        assert_eq!(c.value, 2.0);
        let current = c.header().next.unwrap();

        let c = unsafe { &*(current.as_ptr() as *const Integer) };
        assert_eq!(c.header().obj_type, Integer::r#type());
        assert_eq!(c.value, 1);
        let current = c.header().next;

        assert!(current.is_none(), "Heap has more elements than expected");
    }

    #[test]
    fn test_heap_drop() {
        let mut heap = GcHeap::default();

        heap.alloc(Integer::new(1)).unwrap();
        heap.alloc(Float::new(2.0)).unwrap();
        heap.alloc(Integer::new(3)).unwrap();

        drop(heap); // simulate dropping the heap

        // no assertions needed --- if it doesn't crash, the test passes
    }
}
