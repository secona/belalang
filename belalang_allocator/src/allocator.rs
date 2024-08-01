use std::ptr::NonNull;

pub enum AllocError {
    BadRequest,
    OutOfMemory,
}

#[derive(PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

pub enum Mark {
    Allocated,
    Unmarked,
    Marked,
}

pub enum AllocType {
    String,
}

pub trait AllocObject {
    const TYPE_ID: AllocType;
}

pub trait AllocRaw {
    type Header: AllocHeader;

    fn alloc(&self, object: impl AllocObject) -> Result<NonNull<impl AllocObject>, AllocError>;
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
