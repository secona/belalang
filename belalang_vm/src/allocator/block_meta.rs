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

    pub fn find_next_available_hole(
        &self,
        starting_at: usize,
        alloc_size: usize,
    ) -> Option<(usize, usize)> {
        let mut count = 0;
        let starting_line = starting_at / LINE_SIZE;
        let lines_required = (alloc_size + LINE_SIZE - 1) / LINE_SIZE;

        let mut end = starting_line;

        for index in (0..starting_line).rev() {
            if unsafe { *self.lines.add(index) } == 0 {
                count += 1;

                if index == 0 && count >= lines_required {
                    let limit = index * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }
            } else {
                if count > lines_required {
                    let limit = (index + 2) * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }

                count = 0;
                end = index;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::allocator::block::Block;

    use super::{BlockMeta, BLOCK_SIZE, LINE_SIZE};

    #[test]
    fn find_next_hole() {
        let block = Block::new(BLOCK_SIZE).unwrap();
        let mut meta = BlockMeta::new(block.as_ptr());

        meta.mark_line(0);
        meta.mark_line(1);
        meta.mark_line(2);
        meta.mark_line(4);
        meta.mark_line(10);

        let expect = Some((10 * LINE_SIZE, 6 * LINE_SIZE));
        let got = meta.find_next_available_hole(10 * LINE_SIZE, LINE_SIZE);

        assert_eq!(expect, got)
    }

    #[test]
    fn find_next_hole_at_zero() {
        let block = Block::new(BLOCK_SIZE).unwrap();
        let mut meta = BlockMeta::new(block.as_ptr());

        meta.mark_line(3);
        meta.mark_line(4);
        meta.mark_line(5);

        let expect = Some((3 * LINE_SIZE, 0));
        let got = meta.find_next_available_hole(3 * LINE_SIZE, LINE_SIZE);

        assert_eq!(expect, got);
    }
}
