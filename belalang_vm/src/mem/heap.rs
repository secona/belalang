use std::alloc::{alloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::errors::RuntimeError;
use crate::types::BelalangType;

/* The implementation is still wrong. Currently, the heap can only store one
 * type of value. This is clearly not the end goal: the heap needs to be able
 * to store multiple types of values.
 *
 * I think the BelalangType implementation can be at blame here. Using trait
 * objects is a bit tricky to say the least. I am thinking of using dynamic
 * dispatch, where the alloc function is implemented as a struct method.
 * This approach makes sense considering each type may have different ways
 * to allocate their memory.
 */

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub marked: bool,
}

#[derive(Debug, PartialEq)]
pub struct Object<T: BelalangType> {
    pub header: ObjectHeader,
    pub data: T,
    pub next: Option<NonNull<Object<T>>>,
}

pub struct Heap<T: BelalangType> {
    pub start: Option<NonNull<Object<T>>>,
    _marker: PhantomData<T>,
}

#[allow(clippy::derivable_impls)]
impl<T: BelalangType> Default for Heap<T> {
    fn default() -> Self {
        Self {
            start: None,
            _marker: PhantomData,
        }
    }
}

impl<T: BelalangType> Heap<T> {
    pub fn alloc(&mut self, data: T) -> Result<(), RuntimeError> {
        let layout = Layout::new::<Object<T>>();

        let object_ptr = unsafe {
            let object_ptr = alloc(layout) as *mut Object<T>;

            if object_ptr.is_null() {
                return Err(RuntimeError::AllocationFailed);
            }

            object_ptr.write(Object {
                header: ObjectHeader {
                    marked: false,
                },
                data,
                next: self.start,
            });

            NonNull::new_unchecked(object_ptr)
        };

        self.start = Some(object_ptr);

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use test_case::test_case;

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
            let object = unsafe { current.unwrap().read() };

            assert_eq!(object.header.marked, false);
            assert_eq!(object.data.value, *d);

            current = object.next;
        }

        assert!(current.is_none());
    }
}
