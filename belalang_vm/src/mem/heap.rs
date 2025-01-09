use std::alloc::{alloc, Layout};
use std::ptr::NonNull;

use belalang_devel::errors::RuntimeError;

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub marked: bool,
    pub type_id: u32,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub header: ObjectHeader,
    pub next: Option<NonNull<Object>>,
}

pub struct Heap {
    pub start: Option<NonNull<Object>>,
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
    pub fn alloc(&mut self, object_type_id: u32) -> Result<(), RuntimeError> {
        let layout = Layout::new::<Object>();

        let object_ptr = unsafe {
            let object_ptr = alloc(layout) as *mut Object;

            if object_ptr.is_null() {
                return Err(RuntimeError::AllocationFailed);
            }

            (*object_ptr).header.type_id = object_type_id;
            (*object_ptr).next = self.start;

            NonNull::new_unchecked(object_ptr)
        };

        self.start = Some(object_ptr);

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn heap_alloc() {
        let mut heap = Heap::default();

        heap.alloc(123).unwrap();

        assert!(heap.start.is_some());

        let object = unsafe { heap.start.unwrap().read() };
        assert_eq!(object.header.type_id, 123);
        assert_eq!(object.header.marked, false);
    }
}
