use std::cell::UnsafeCell;
use std::marker::PhantomData;

use crate::block_meta::BLOCK_CAPACITY;
use crate::bump_block::BumpBlock;

enum AllocError {
    BadRequest,
}

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

#[derive(PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
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
