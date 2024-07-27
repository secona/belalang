use crate::object::Object;

#[derive(Debug, Default)]
pub struct Frame {
    pub slots: Vec<Object>,
    pub ret_addr: usize,
}

impl Frame {
    pub fn new(ret_addr: usize, locals: Vec<Object>) -> Self {
        Self {
            ret_addr,
            slots: locals,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct FrameStack {
    frames: Vec<Frame>,
}

impl FrameStack {
    pub fn top_level(&self) -> bool {
        self.frames.len() == 0
    }

    fn current(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn current_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn set_local(&mut self, index: usize, object: Object) {
        let frame = self.current_mut();
        match frame.slots.get(index) {
            Some(_) => frame.slots[index] = object,
            None => frame.slots.insert(index, object),
        }
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
