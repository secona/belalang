use std::alloc::{self, Layout, LayoutError};
use std::ptr::NonNull;

pub struct Block {
    ptr: NonNull<u8>,
    size: usize,
}

impl Block {
    pub fn new(size: usize) -> Result<Block, LayoutError> {
        let layout = Layout::from_size_align(size, size)?;

        let ptr = unsafe { NonNull::new_unchecked(alloc::alloc(layout)) };
        Ok(Block { ptr, size })
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, self.size);
            alloc::dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn initialization() {
        assert!(Block::new(4096).is_ok());
    }
}
