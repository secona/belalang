use std::ptr::NonNull;
use std::marker::PhantomData;
use std::cell::Cell;
use std::alloc::{alloc, Layout};

use crate::{GcPtr, MemoryError};

pub struct GcObjectHeader {
    pub obj_type: u32,
    pub ref_count: Cell<usize>,
    pub is_marked: bool,
    pub next: Option<NonNull<GcObjectHeader>>,
}

pub trait GcObject {
    fn header(&self) -> &GcObjectHeader;
    fn header_mut(&mut self) -> &mut GcObjectHeader;

    fn type_name() -> String
    where
        Self: Sized;
}

#[derive(Default)]
pub struct GcHeap {
    pub start: Option<NonNull<GcObjectHeader>>,
    _marker: PhantomData<GcObjectHeader>,
}

impl GcHeap {
    pub fn alloc<T: GcObject>(&mut self, object: T) -> Result<GcPtr, MemoryError> {
        let layout = Layout::new::<T>();

        let base_ptr: *mut T = unsafe {
            let ptr = alloc(layout) as *mut T;

            if ptr.is_null() {
                return Err(MemoryError::AllocationFailed);
            }

            ptr.write(object);

            ptr
        };

        unsafe {
            let ptr = base_ptr as *mut GcObjectHeader;
            (*ptr).next = self.start;

            self.start = Some(NonNull::new_unchecked(ptr));
        }

        Ok(GcPtr::new(unsafe {
            NonNull::new_unchecked(std::mem::transmute::<
                *mut (dyn GcObject + '_),
                *mut (dyn GcObject + 'static),
            >(base_ptr))
        }))
    }
}

impl Drop for GcHeap {
    fn drop(&mut self) {
        self.start = None;
    }
}
