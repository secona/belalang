use std::cell::Cell;
use std::ptr::NonNull;

use super::BelalangObject;

#[derive(Debug)]
#[repr(C)]
pub struct BelalangBase {
    pub obj_type: u32,
    pub ref_count: Cell<usize>,
    pub is_marked: bool,
    pub next: Option<NonNull<BelalangBase>>,
}

impl BelalangBase {
    pub fn new<T: BelalangObject>() -> Self {
        Self {
            obj_type: T::r#type(),
            ref_count: Cell::new(0),
            is_marked: false,
            next: None,
        }
    }
}
