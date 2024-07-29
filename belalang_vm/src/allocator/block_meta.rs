pub const BLOCK_SIZE: usize = 1 << 15;
pub const LINE_SIZE: usize = 1 << 7;

pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;
pub const BLOCK_CAPACITY: usize = BLOCK_SIZE - LINE_COUNT;

pub struct BlockMeta {
    lines: *mut u8,
}

impl BlockMeta {
    pub fn new(block_ptr: *const u8) -> Self {
        let mut bm = Self {
            lines: unsafe { block_ptr.add(BLOCK_CAPACITY) as *mut u8 },
        };

        bm.reset();

        bm
    }

    unsafe fn as_block_mark(&mut self) -> &mut u8 {
        &mut *self.lines.add(LINE_COUNT - 1)
    }

    unsafe fn as_line_mark(&mut self, index: usize) -> &mut u8 {
        &mut *self.lines.add(index)
    }

    pub fn mark_line(&mut self, index: usize) {
        unsafe { *self.as_line_mark(index) = 1 };
    }

    pub fn mark_block(&mut self) {
        unsafe { *self.as_block_mark() = 1 };
    }

    pub fn reset(&mut self) {
        unsafe {
            for i in 0..LINE_COUNT {
                *self.lines.add(i) = 0;
            }
        }
    }
}
