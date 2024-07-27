use crate::object::{Function, Object};

#[derive(Debug, Default)]
pub struct Frame {
    pub slots: Vec<Object>,
    pub ret_addr: usize,
}

impl Frame {
    pub fn new(ret_addr: usize, function: Function, init_locals: Vec<Object>) -> Self {
        let mut slots = vec![Object::Null; function.locals_count];
        slots.splice(..init_locals.len(), init_locals);

        Self { ret_addr, slots }
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
        frame.slots[index] = object;
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
