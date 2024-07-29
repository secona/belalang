use std::mem::size_of;
use std::ptr::NonNull;

use super::block::Block;
use super::block_meta::{BlockMeta, BLOCK_CAPACITY, BLOCK_SIZE};

pub struct BumpBlock {
    cursor: NonNull<u8>,
    limit: NonNull<u8>,
    block: Block,
    meta: BlockMeta,
}

impl BumpBlock {
    pub fn new() -> Self {
        let block = Block::new(BLOCK_SIZE).unwrap();
        let ptr = block.as_ptr();
        let meta = BlockMeta::new(ptr);

        let cursor = unsafe { NonNull::new_unchecked(ptr.add(BLOCK_CAPACITY)) };
        let limit = unsafe { NonNull::new_unchecked(ptr) };

        Self {
            cursor,
            limit,
            block,
            meta,
        }
    }

    pub fn alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let start_ptr = self.block.as_ptr() as usize;
        let cursor_ptr = self.cursor.as_ptr() as usize;

        let align_mask = !(size_of::<usize>() - 1);

        let next_ptr: usize = cursor_ptr.checked_sub(alloc_size)? & align_mask;

        if next_ptr < start_ptr {
            let block_relative_limit =
                unsafe { self.limit.as_ptr().sub(self.block.as_ptr() as usize) } as usize;

            if block_relative_limit > 0 {
                if let Some((cursor, limit)) = self
                    .meta
                    .find_next_available_hole(block_relative_limit, alloc_size)
                {
                    self.cursor =
                        unsafe { NonNull::new_unchecked(self.block.as_ptr().add(cursor)) };
                    self.limit = unsafe { NonNull::new_unchecked(self.block.as_ptr().add(limit)) };
                    return self.alloc(alloc_size);
                }
            }

            None
        } else {
            self.cursor = unsafe { NonNull::new_unchecked(next_ptr as *mut u8) };
            Some(next_ptr as *const u8)
        }
    }
}
