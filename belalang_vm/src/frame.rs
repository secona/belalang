use crate::object::Object;

#[derive(Debug, Default)]
pub struct Frame {
    pub slots: Vec<Object>,
    pub ret_addr: usize,
}

#[derive(Debug, Default)]
pub struct FrameStack {
    main_frame: Frame,
    frames: Vec<Frame>,
}

impl FrameStack {
    pub fn top_level(&self) -> bool {
        self.frames.len() == 0
    }

    fn current(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.main_frame)
    }

    fn current_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap_or(&mut self.main_frame)
    }

    pub fn set_local(&mut self, index: usize, object: Object) {
        let frame = self.current_mut();
        match frame.slots.get(index) {
            Some(_) => frame.slots[index] = object,
            None => frame.slots.insert(index, object),
        }
    }

    pub fn set_locals(&mut self, objects: Vec<Object>) {
        self.current_mut().slots.extend(objects);
    }

    pub fn get_local(&self, index: usize) -> Object {
        self.current().slots[index].clone()
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }
}
