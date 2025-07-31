use std::alloc::{Layout, alloc};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::BelalangPtr;
use crate::errors::RuntimeError;
use crate::objects::{BelalangBase, BelalangObject};

/// Belalang VM's heap implementation
///
/// This is a GC-managed heap allocator for Belalang VM. It works by having the
/// objects in a linked list.
//
/// Note that this is a very early implementation. Breaking changes will be
/// made.
pub struct Heap {
    pub start: Option<NonNull<BelalangBase>>,
    _marker: PhantomData<BelalangBase>,
}

#[allow(clippy::derivable_impls)]
impl Default for Heap {
    fn default() -> Self {
        Self {
            start: None,
            _marker: PhantomData,
        }
    }
}

impl Heap {
    /// Allocate function for Belalang VM heap
    ///
    /// It allocates the Belalang object into the linked list by inserting it at
    /// the front. It returns the address to the allocated object. Note that
    /// this function not only allocates memory but also write to the newly
    /// allocated memory.
    pub fn alloc<T: BelalangObject>(&mut self, object: T) -> Result<BelalangPtr, RuntimeError> {
        let layout = Layout::new::<T>();

        let base_ptr: *mut T = unsafe {
            let ptr = alloc(layout) as *mut T;

            if ptr.is_null() {
                return Err(RuntimeError::AllocationFailed);
            }

            ptr.write(object);

            ptr
        };

        unsafe {
            let ptr = base_ptr as *mut BelalangBase;
            (*ptr).next = self.start;

            self.start = Some(NonNull::new_unchecked(ptr));
        }

        Ok(BelalangPtr::new(unsafe {
            NonNull::new_unchecked(std::mem::transmute::<
                *mut (dyn BelalangObject + '_),
                *mut (dyn BelalangObject + 'static),
            >(base_ptr))
        }))
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        self.start = None;
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;
    use crate::objects::BelalangObject;
    use crate::objects::boolean::BelalangBoolean;
    use crate::objects::integer::BelalangInteger;

    /// Test value that can be allocated on the heap
    enum TestValue {
        Integer(i64),
        Boolean(bool),
    }

    impl TestValue {
        /// Allocate this value on the heap
        fn allocate(&self, heap: &mut Heap) -> BelalangPtr {
            match self {
                TestValue::Integer(val) => heap.alloc(BelalangInteger::new(*val)).unwrap(),
                TestValue::Boolean(val) => heap.alloc(BelalangBoolean::new(*val)).unwrap(),
            }
        }

        /// Verify that the pointer points to this value
        fn verify(&self, ptr: &NonNull<BelalangBase>) {
            let base = unsafe { &*ptr.as_ptr() };

            match self {
                TestValue::Integer(expected) => {
                    assert_eq!(base.obj_type, BelalangInteger::r#type());
                    let integer = unsafe { &*(ptr.as_ptr() as *const BelalangInteger) };
                    assert_eq!(integer.value, *expected);
                },
                TestValue::Boolean(expected) => {
                    assert_eq!(base.obj_type, BelalangBoolean::r#type());
                    let boolean = unsafe { &*(ptr.as_ptr() as *const BelalangBoolean) };
                    assert_eq!(boolean.value, *expected);
                },
            }
        }
    }

    /// Helper function to verify the entire heap structure matches expected
    /// values
    fn verify_heap_structure(heap: &Heap, expected_values: &[TestValue]) {
        let mut current = heap.start;

        // Check each object in the heap matches the expected values (in reverse order)
        for (i, expected) in expected_values.iter().rev().enumerate() {
            let Some(ptr) = current else {
                panic!("Heap has fewer elements than expected at position {i}");
            };

            expected.verify(&ptr);
            current = unsafe { &*ptr.as_ptr() }.next;
        }

        // Ensure we've reached the end of the heap
        assert!(current.is_none(), "Heap has more elements than expected");
    }

    /// Helper function to verify a vector of pointers to the Heap
    fn verify_heap_pointer_equality(heap: &Heap, ptrs: Vec<BelalangPtr>) {
        let mut current = heap.start;
        for (i, ptr) in ptrs.iter().rev().enumerate() {
            let Some(c) = current else {
                panic!("Error: Unexpected None at position {i}");
            };

            assert_eq!(
                c.as_ptr() as *const (),
                ptr.as_ptr() as *const (),
                "Pointer mismatch at position {i}"
            );

            current = unsafe { &*c.as_ptr() }.next;
        }
    }

    #[test]
    fn test_heap_allocations_1() {
        // Test cases with their expected heap structures
        let test_case = vec![TestValue::Integer(1), TestValue::Integer(2), TestValue::Integer(3)];

        let mut heap = Heap::default();
        let mut ptrs = Vec::new();

        // Allocate all values
        for value in &test_case {
            let ptr = value.allocate(&mut heap);
            ptrs.push(ptr);
        }

        // Verify heap structure
        verify_heap_structure(&heap, &test_case);

        // Also verify pointer equality
        verify_heap_pointer_equality(&heap, ptrs);
    }

    #[test]
    fn test_heap_allocations_2() {
        // Test cases with their expected heap structures
        let test_case = vec![TestValue::Integer(1), TestValue::Boolean(false), TestValue::Integer(3)];

        let mut heap = Heap::default();
        let mut ptrs = Vec::new();

        // Allocate all values
        for value in &test_case {
            let ptr = value.allocate(&mut heap);
            ptrs.push(ptr);
        }

        // Verify heap structure
        verify_heap_structure(&heap, &test_case);

        // Also verify pointer equality
        verify_heap_pointer_equality(&heap, ptrs);
    }

    #[test]
    fn test_heap_deallocation() {
        let test_case = vec![TestValue::Integer(1), TestValue::Boolean(true), TestValue::Integer(42)];

        let mut heap = Heap::default();
        let mut ptrs = Vec::new();

        // Allocate all values
        for value in &test_case {
            let ptr = value.allocate(&mut heap);
            ptrs.push(ptr);
        }

        // Verify heap structure
        verify_heap_structure(&heap, &test_case);

        // Drop each pointer one by one
        for ptr in ptrs {
            drop(ptr);
        }
    }

    #[test]
    fn test_heap_drop() {
        let mut heap = Heap::default();

        heap.alloc(BelalangInteger::new(1)).unwrap();
        heap.alloc(BelalangBoolean::new(true)).unwrap();

        drop(heap); // simulate dropping the heap

        // no assertions needed --- if it doesn't crash, the test passes
    }
}
