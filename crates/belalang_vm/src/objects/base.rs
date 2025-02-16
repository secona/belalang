use std::ptr::NonNull;

use super::BelalangObject;

#[derive(Debug)]
pub struct BelalangBase {
    pub obj_type: u32,
    pub is_marked: bool,
    pub next: Option<NonNull<BelalangBase>>,
}

impl BelalangBase {
    pub fn new<T: BelalangObject>() -> Self {
        Self {
            obj_type: T::r#type(),
            is_marked: false,
            next: None,
        }
    }
}
