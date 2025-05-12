use std::alloc::{Layout, alloc};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::BelalangBase;
use crate::core::BelalangPtr;
use crate::errors::RuntimeError;
use crate::objects::BelalangObject;

/// Belalang VM's heap implementation
/// 
/// This is a GC-managed heap allocator for Belalang VM. It works by having the objects in a linked
/// list.
//
/// Note that this is a very early implementation. Breaking changes will be made. 
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
    /// It allocates the Belalang object into the linked list by inserting it at the front. It
    /// returns the address to the allocated object. Note that this function not only allocates
    /// memory but also write to the newly allocated memory.
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
    use crate::objects::BelalangObject;
    use crate::objects::boolean::BelalangBoolean;
    use crate::objects::integer::BelalangInteger;

    use super::*;

    fn test_heap_alloc(data: Vec<i64>) {
        let mut heap = Heap::default();
        let mut allocated_ptrs = Vec::new();

        for i in &data {
            let value = BelalangInteger::new(*i);
            let ptr = heap.alloc(value).unwrap();
            allocated_ptrs.push(ptr);
        }

        let mut current = heap.start;
        for (i, (d, ptr)) in data
            .iter()
            .rev()
            .zip(allocated_ptrs.iter().rev())
            .enumerate()
        {
            let Some(c) = current else {
                panic!("Error: Unexpected None at heap.start");
            };

            assert_eq!(
                c.as_ptr() as *const (),
                ptr.as_ptr() as *const (),
                "Pointer mismatch at position {}",
                i
            );

            let base = unsafe { &*c.as_ptr() };

            assert_eq!(base.obj_type, BelalangInteger::r#type());

            let integer = unsafe {
                let ptr = c.as_ptr() as *const BelalangInteger;
                ptr.read()
            };

            assert_eq!(integer.value, *d);

            current = base.next;
        }

        assert!(current.is_none());
    }

    #[test]
    fn heap_alloc_simple() {
        test_heap_alloc(vec![1, 2, 3]);
    }

    #[derive(Debug)]
    enum Type {
        Integer(i64),
        Boolean(bool),
    }

    fn test_heap_alloc_multiple_types(data: Vec<Type>) {
        let mut heap = Heap::default();
        let mut allocated_ptrs = Vec::new();

        for i in &data {
            let ptr = match i {
                Type::Integer(integer) => {
                    let value = BelalangInteger::new(*integer);
                    heap.alloc(value).unwrap()
                }
                Type::Boolean(boolean) => {
                    let value = BelalangBoolean::new(*boolean);
                    heap.alloc(value).unwrap()
                }
            };
            allocated_ptrs.push(ptr);
        }

        let mut current = heap.start;

        for (i, (d, ptr)) in data
            .iter()
            .rev()
            .zip(allocated_ptrs.into_iter().rev())
            .enumerate()
        {
            let Some(c) = current else {
                panic!("Error: Unexpected None at heap.start");
            };

            assert_eq!(
                c.as_ptr() as *const (),
                ptr.as_ptr() as *const (),
                "Pointer mismatch at position {}",
                i
            );

            let base = unsafe { &*c.as_ptr() };

            match d {
                Type::Integer(integer) => {
                    let object = unsafe { &*(c.as_ptr() as *const BelalangInteger) };
                    assert_eq!(base.obj_type, BelalangInteger::r#type());
                    assert_eq!(object.value, *integer);
                }
                Type::Boolean(boolean) => {
                    let object = unsafe { &*(c.as_ptr() as *const BelalangBoolean) };
                    assert_eq!(base.obj_type, BelalangBoolean::r#type());
                    assert_eq!(object.value, *boolean);
                }
            };

            current = base.next;
        }

        assert!(current.is_none());
    }

    #[test]
    fn heap_alloc_multiple_types_case_1() {
        test_heap_alloc_multiple_types(vec![Type::Integer(1), Type::Boolean(true)]);
    }

    #[test]
    fn heap_alloc_multiple_types_case_2() {
        test_heap_alloc_multiple_types(vec![
            Type::Integer(1),
            Type::Boolean(true),
            Type::Boolean(false),
        ]);
    }

    #[test]
    fn heap_alloc_multiple_types_case_3() {
        test_heap_alloc_multiple_types(vec![
            Type::Integer(1),
            Type::Boolean(true),
            Type::Integer(100),
        ]);
    }

    fn test_heap_dealloc(data: Vec<i64>) {
        let mut heap = Heap::default();
        let mut allocated_ptrs = Vec::new();

        for i in &data {
            let value = BelalangInteger::new(*i);
            let ptr = heap.alloc(value).unwrap();
            allocated_ptrs.push(ptr);
        }

        let mut current = heap.start;

        for (i, (d, ptr)) in data
            .iter()
            .rev()
            .zip(allocated_ptrs.into_iter().rev())
            .enumerate()
        {
            let Some(c) = current else {
                panic!("Error: Unexpected None at heap.start");
            };

            assert_eq!(
                c.as_ptr() as *const (),
                ptr.as_ptr() as *const (),
                "Pointer mismatch at position {}",
                i
            );

            let base = unsafe { &*c.as_ptr() };
            assert_eq!(base.obj_type, BelalangInteger::r#type());

            let integer = unsafe { &*(c.as_ptr() as *const BelalangInteger) };
            assert_eq!(integer.value, *d);

            drop(ptr);

            current = base.next;
        }

        assert!(current.is_none());
    }

    #[test]
    fn heap_dealloc_case_1() {
        test_heap_dealloc(vec![1, 2, 3]);
    }

    #[test]
    fn heap_drop() {
        let mut heap = Heap::default();
        heap.alloc(BelalangInteger::new(1)).unwrap();
        drop(heap);
    }
}
