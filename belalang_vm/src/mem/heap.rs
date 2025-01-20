use std::alloc::{alloc, Layout};
use std::ptr::NonNull;

use crate::errors::RuntimeError;
use crate::types::BelalangObject;

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

pub struct Heap {
    pub start: Option<NonNull<BelalangObject>>,
}

#[allow(clippy::derivable_impls)]
impl Default for Heap {
    fn default() -> Self {
        Self {
            start: None,
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
}
