use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::errors::RuntimeError;
use crate::types::BelalangObject;

pub struct Heap {
    pub start: Option<NonNull<BelalangObject>>,
    _marker: PhantomData<BelalangObject>,
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
    pub fn alloc<T>(&mut self, object: T) -> Result<(), RuntimeError> {
        let layout = Layout::new::<T>();

        let object_ptr = unsafe {
            let object_ptr = alloc(layout) as *mut T;

            if object_ptr.is_null() {
                return Err(RuntimeError::AllocationFailed);
            }

            object_ptr.write(object);

            let ptr = object_ptr as *mut BelalangObject;
            (*ptr).next = self.start;

            NonNull::new_unchecked(object_ptr as *mut BelalangObject)
        };

        self.start = Some(object_ptr);

        Ok(())
    }

    pub fn dealloc<T>(&mut self, ptr: NonNull<T>) -> Result<(), RuntimeError> {
        let layout = Layout::new::<T>();

        let object_ptr = ptr.as_ptr() as *mut BelalangObject;

        unsafe {
            if let Some(start) = self.start {
                if start.as_ptr() == object_ptr {
                    self.start = (*object_ptr).next;
                } else {
                    let mut current = start;
                    while let Some(next) = (*current.as_ptr()).next {
                        if next.as_ptr() == object_ptr {
                            (*current.as_ptr()).next = (*object_ptr).next;
                            break;
                        }
                        current = next;
                    }
                }
            }

            dealloc(ptr.as_ptr() as *mut u8, layout);
        }

        Ok(())
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        while let Some(ptr) = self.start {
            unsafe {
                let layout = Layout::new::<BelalangObject>();
                self.start = (*ptr.as_ptr()).next;
                dealloc(ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use test_case::test_case;

    use crate::types::BelalangType;
    use crate::types::integer::BelalangInteger;

    use super::*;

    #[test_case(vec![1, 2, 3])]
    fn heap_alloc(data: Vec<i64>) {
        let mut heap = Heap::default();

        for i in &data {
            let value = BelalangInteger::new(*i);
            heap.alloc(value).unwrap();
        }

        let mut current = heap.start;

        for d in data.iter().rev() {
            let Some(c) = current else {
                panic!("Error: Unexpected None at heap.start");
            };

            let object = unsafe {
                let ptr = c.as_ptr() as *const BelalangObject;
                ptr.read()
            };

            assert_eq!(object.obj_type, BelalangInteger::r#type());

            let integer = unsafe {
                let ptr = c.as_ptr() as *const BelalangInteger;
                ptr.read()
            };

            assert_eq!(integer.value, *d);

            current = object.next;
        }

        assert!(current.is_none());
    }

    #[test_case(vec![1, 2, 3])]
    fn heap_dealloc(data: Vec<i64>) {
        let mut heap = Heap::default();

        for i in &data {
            let value = BelalangInteger::new(*i);
            heap.alloc(value).unwrap();
        }

        let mut current = heap.start;

        for d in data.iter().rev() {
            let Some(c) = current else {
                panic!("Error: Unexpected None at heap.start");
            };

            let object = unsafe {
                let ptr = c.as_ptr() as *const BelalangObject;
                ptr.read()
            };

            assert_eq!(object.obj_type, BelalangInteger::r#type());

            let integer = unsafe {
                let ptr = c.as_ptr() as *const BelalangInteger;
                ptr.read()
            };

            assert_eq!(integer.value, *d);
            
            heap.dealloc(c).unwrap();

            current = object.next;
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
