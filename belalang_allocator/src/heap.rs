pub mod allocator;
pub mod block;
pub mod block_meta;
pub mod bump_block;

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

use self::allocator::{AllocError, AllocHeader, AllocObject, AllocRaw, Mark, Size};
use self::block_meta::BLOCK_CAPACITY;
use self::bump_block::BumpBlock;

#[derive(Default)]
struct BlockList {
    head: Option<BumpBlock>,
    overflow: Option<BumpBlock>,
    rest: Vec<BumpBlock>,
}

impl BlockList {
    fn overflow_alloc(&mut self, alloc_size: usize) -> Result<*const u8, AllocError> {
        assert!(alloc_size <= BLOCK_CAPACITY);

        let ptr = match self.overflow {
            Some(ref mut overflow) => match overflow.alloc(alloc_size) {
                Some(ptr) => ptr,
                None => {
                    let previous = std::mem::replace(overflow, BumpBlock::new());
                    self.rest.push(previous);
                    overflow.alloc(alloc_size).unwrap()
                }
            },
            None => {
                let mut overflow = BumpBlock::new();
                let ptr = overflow.alloc(alloc_size).unwrap();
                self.overflow = Some(overflow);
                ptr
            }
        };

        Ok(ptr)
    }
}

pub struct Heap<H> {
    blocks: UnsafeCell<BlockList>,
    _marker: PhantomData<*const H>,
}

impl<H> Default for Heap<H> {
    fn default() -> Self {
        Self {
            blocks: UnsafeCell::new(BlockList::default()),
            _marker: PhantomData,
        }
    }
}

impl<H> Heap<H> {
    fn find_space(&self, alloc_size: usize, size: Size) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get() };

        if size == Size::Large {
            return Err(AllocError::BadRequest);
        }

        let ptr = match blocks.head {
            Some(ref mut head) => {
                if size == Size::Medium && alloc_size > head.current_hole_size() {
                    return blocks.overflow_alloc(alloc_size);
                }

                match head.alloc(alloc_size) {
                    Some(ptr) => ptr,
                    None => {
                        let previous = std::mem::replace(head, BumpBlock::new());
                        blocks.rest.push(previous);
                        head.alloc(alloc_size).unwrap()
                    }
                }
            }
            None => {
                let mut head = BumpBlock::new();
                let ptr = head.alloc(alloc_size).unwrap();
                blocks.head = Some(head);
                ptr
            }
        };

        Ok(ptr)
    }
}

impl<H: AllocHeader> AllocRaw for Heap<H> {
    type Header = H;

    fn alloc<T>(&self, object: T) -> Result<NonNull<T>, AllocError>
    where
        T: AllocObject,
    {
        let header_size = std::mem::size_of::<H>();
        let object_size = std::mem::size_of::<T>();
        let total_size = header_size + object_size;

        let align = std::mem::size_of::<usize>();
        let alloc_size = (total_size + (align - 1)) & !(align - 1);

        let size = Size::get_size(alloc_size)?;

        let ptr = self.find_space(alloc_size, size)? as *mut Self::Header;
        let header = Self::Header::new::<T>(object_size, size, Mark::Allocated);
        unsafe { std::ptr::write(ptr, header) };

        let object_ptr = unsafe { ptr.offset(header_size as isize) } as *mut T;
        unsafe { std::ptr::write(object_ptr, object) };

        unsafe { Ok(NonNull::new_unchecked(object_ptr)) }
    }

    fn alloc_array(&self, array_size: usize) -> Result<NonNull<u8>, AllocError> {
        let header_size = std::mem::size_of::<H>();
        let total_size = header_size + array_size;

        let align = std::mem::size_of::<usize>();
        let alloc_size = (total_size + (align - 1)) & !(align - 1);

        let size = Size::get_size(alloc_size)?;

        let ptr = self.find_space(alloc_size, size)? as *mut Self::Header;
        let header = Self::Header::new_array(array_size, size, Mark::Allocated);
        unsafe { std::ptr::write(ptr, header) };

        let array_space = unsafe { ptr.offset(header_size as isize) } as *mut u8;
        let array = unsafe { std::slice::from_raw_parts_mut(array_space, array_size) };

        for byte in array {
            *byte = 0;
        }

        unsafe { Ok(NonNull::new_unchecked(array_space)) }
    }

    fn get_header(object: NonNull<()>) -> NonNull<Self::Header> {
        unsafe { NonNull::new_unchecked(object.cast::<Self::Header>().as_ptr().offset(-1)) }
    }

    fn get_object(header: NonNull<Self::Header>) -> NonNull<()> {
        unsafe { NonNull::new_unchecked(header.as_ptr().offset(1).cast::<()>()) }
    }
}

#[cfg(test)]
mod tests {
    use super::allocator::{AllocHeader, AllocObject, AllocRaw, AllocType, Mark, Size};
    use super::Heap;

    struct TestHeader {
        _size: Size,
        _mark: Mark,
        type_id: AllocType,
        _size_bytes: usize,
    }

    impl AllocHeader for TestHeader {
        fn new<O: AllocObject>(size: usize, size_class: Size, mark: Mark) -> Self {
            TestHeader {
                _size: size_class,
                _mark: mark,
                type_id: O::TYPE_ID,
                _size_bytes: size,
            }
        }

        fn new_array(array_size: usize, size_class: Size, mark: Mark) -> Self {
            TestHeader {
                _size: size_class,
                _mark: mark,
                type_id: AllocType::Array,
                _size_bytes: array_size,
            }
        }

        fn mark(&mut self) {}

        fn is_marked(&self) -> bool {
            self._mark == Mark::Marked
        }

        fn size_class(&self) -> Size {
            self._size
        }

        fn size(&self) -> u32 {
            self._size_bytes as u32
        }

        fn type_id(&self) -> AllocType {
            self.type_id
        }
    }

    impl AllocObject for String {
        const TYPE_ID: AllocType = AllocType::String;
    }

    impl AllocObject for i32 {
        const TYPE_ID: AllocType = AllocType::Integer;
    }

    #[test]
    fn test_mem() {
        let heap = Heap::<TestHeader>::default();

        match heap.alloc(String::from("test")) {
            Ok(s) => {
                let value = unsafe { s.as_ref() };
                assert_eq!(*value, String::from("test"));
            }
            Err(_) => panic!("alloc failed."),
        }

        match heap.alloc(123123) {
            Ok(s) => {
                let value = unsafe { s.as_ref() };
                assert_eq!(*value, 123123);
            }
            Err(_) => panic!("alloc failed."),
        }
    }

    #[test]
    fn test_array() {
        let heap = Heap::<TestHeader>::default();

        match heap.alloc_array(64) {
            Ok(ptr) => {
                let ptr = ptr.as_ptr();
                let array = unsafe { std::slice::from_raw_parts(ptr, 64) };
                assert_eq!(array, &[0; 64]);
            }
            Err(_) => panic!("alloc failed."),
        }
    }
}
