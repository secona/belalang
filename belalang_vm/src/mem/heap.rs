use std::alloc::{alloc, Layout};
use std::ptr::null_mut;

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub marked: bool,
    pub type_id: u32,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub header: ObjectHeader,
    pub next: *mut Object,
}

pub struct Heap {
    pub start: *mut Object,
}

impl Default for Heap {
    fn default() -> Self {
        Self {
            start: null_mut(),
        }
    }
}

impl Heap {
    pub fn alloc(&mut self, object_type_id: u32) {
        let layout = Layout::new::<Object>();

        let object_ptr = unsafe {
            let object_ptr = alloc(layout) as *mut Object;
            (*object_ptr).header.type_id = object_type_id;
            (*object_ptr).next = self.start;
            object_ptr
        };

        self.start = object_ptr;
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn heap_alloc() {
        let mut heap = Heap::default();

        heap.alloc(123);

        assert!(!heap.start.is_null());

        let object = unsafe { heap.start.read() };
        assert_eq!(object.header.type_id, 123);
        assert_eq!(object.header.marked, false);
    }
}
