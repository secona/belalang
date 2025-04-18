use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::errors::RuntimeError;
use crate::objects::ptr::BelalangPtr;
use crate::objects::BelalangObject;
use crate::BelalangBase;

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

    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It deallocates memory pointed to by `ptr`, which must have been previously allocated by this allocator
    /// - The pointer must be valid and properly aligned for type T
    /// - After deallocation, the memory must not be accessed or freed again
    /// - No references to the freed memory may exist after this call
    ///
    /// Caller must ensure:
    /// - The pointer was allocated using this allocator's corresponding allocation method
    /// - The type T matches the type that was originally allocated
    /// - No other parts of the program retain references to this memory after deallocation
    pub unsafe fn dealloc<T>(&mut self, ptr: NonNull<T>) -> Result<(), RuntimeError> {
        unsafe {
            let layout = Layout::new::<T>();

            let base_ptr = ptr.as_ptr() as *mut BelalangBase;

            if let Some(start) = self.start {
                if start.as_ptr() == base_ptr {
                    self.start = (*base_ptr).next;
                } else {
                    let mut current = start;
                    while let Some(next) = (*current.as_ptr()).next {
                        if next.as_ptr() == base_ptr {
                            (*current.as_ptr()).next = (*base_ptr).next;
                            break;
                        }
                        current = next;
                    }
                }
            }

            dealloc(ptr.as_ptr() as *mut u8, layout);

            Ok(())
        }
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        while let Some(ptr) = self.start {
            unsafe {
                self.start = (*ptr.as_ptr()).next;
                self.dealloc::<BelalangBase>(ptr).unwrap();
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use test_case::test_case;

    use crate::objects::boolean::BelalangBoolean;
    use crate::objects::integer::BelalangInteger;
    use crate::objects::BelalangObject;

    use super::*;

    #[test_case(vec![1, 2, 3]; "1")]
    fn heap_alloc(data: Vec<i64>) {
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

            let base = unsafe {
                let ptr = c.as_ptr() as *const BelalangBase;
                ptr.read()
            };

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

    #[derive(Debug)]
    enum Type {
        Integer(i64),
        Boolean(bool),
    }

    #[test_case(vec![Type::Integer(1), Type::Boolean(true)]; "1")]
    #[test_case(vec![Type::Integer(1), Type::Boolean(true), Type::Boolean(false)]; "2")]
    #[test_case(vec![Type::Integer(1), Type::Boolean(true), Type::Integer(100)]; "3")]
    fn heap_alloc_multiple_types(data: Vec<Type>) {
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

            let base = unsafe {
                let ptr = c.as_ptr() as *const BelalangBase;
                ptr.read()
            };

            match d {
                Type::Integer(integer) => {
                    assert_eq!(base.obj_type, BelalangInteger::r#type());

                    let object = unsafe {
                        let ptr = c.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    assert_eq!(object.value, *integer);
                }
                Type::Boolean(boolean) => {
                    assert_eq!(base.obj_type, BelalangBoolean::r#type());

                    let object = unsafe {
                        let ptr = c.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    assert_eq!(object.value, *boolean);
                }
            };

            current = base.next;
        }

        assert!(current.is_none());
    }

    #[test_case(vec![1, 2, 3]; "1")]
    fn heap_dealloc(data: Vec<i64>) {
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

            let base = unsafe {
                let ptr = c.as_ptr() as *const BelalangBase;
                ptr.read()
            };

            assert_eq!(base.obj_type, BelalangInteger::r#type());

            let integer = unsafe {
                let ptr = c.as_ptr() as *const BelalangInteger;
                ptr.read()
            };

            assert_eq!(integer.value, *d);

            unsafe { heap.dealloc(c).unwrap() };

            current = base.next;
        }

        assert!(current.is_none());
    }

    #[test]
    fn heap_drop() {
        let mut heap = Heap::default();
        heap.alloc(BelalangInteger::new(1)).unwrap();
        drop(heap);
    }
}
