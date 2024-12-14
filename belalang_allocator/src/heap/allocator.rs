use std::ptr::NonNull;

use super::block_meta::{BLOCK_CAPACITY, LINE_SIZE};

pub enum AllocError {
    BadRequest,
    OutOfMemory,
}

pub const MAX_ALLOC_SIZE: usize = std::u32::MAX as usize;
pub const SM_MIN: usize = 1;
pub const SM_MAX: usize = LINE_SIZE;
pub const MD_MIN: usize = SM_MAX + 1;
pub const MD_MAX: usize = BLOCK_CAPACITY;
pub const LG_MIN: usize = MD_MAX + 1;
pub const LG_MAX: usize = MAX_ALLOC_SIZE;

#[derive(PartialEq, Clone, Copy)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn get_size(value: usize) -> Result<Self, AllocError> {
        match value {
            SM_MIN..=SM_MAX => Ok(Self::Small),
            MD_MIN..=MD_MAX => Ok(Self::Medium),
            LG_MIN..=LG_MAX => Ok(Self::Large),
            _ => Err(AllocError::BadRequest),
        }
    }
}

#[derive(PartialEq)]
pub enum Mark {
    Allocated,
    Unmarked,
    Marked,
}

#[derive(Clone, Copy)]
pub enum AllocType {
    String,
    Integer,
    Array,
}

pub trait AllocObject {
    const TYPE_ID: AllocType;
}

pub trait AllocRaw {
    type Header: AllocHeader;

    fn alloc<T>(&self, object: T) -> Result<NonNull<T>, AllocError>
    where
        T: AllocObject;
    fn alloc_array(&self, array_size: usize) -> Result<NonNull<u8>, AllocError>;
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header>;
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()>;
}

pub trait AllocHeader: Sized {
    fn new<O: AllocObject>(size: usize, size_class: Size, mark: Mark) -> Self;
    fn new_array(array_size: usize, size_class: Size, mark: Mark) -> Self;
    fn mark(&mut self);
    fn is_marked(&self) -> bool;
    fn size_class(&self) -> Size;
    fn size(&self) -> u32;
    fn type_id(&self) -> AllocType;
}
