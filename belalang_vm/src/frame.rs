use crate::object::{Function, Object};

#[derive(Debug, Default)]
pub struct Frame {
    pub function: Function,
    pub slots: Vec<Object>,
    pub ip: usize,
}

impl Frame {
    pub fn ins(&self) -> &Vec<u8> {
        &self.function.instructions
    }
}

#[derive(Debug, Default)]
pub struct FrameManager {
    pub main_frame: Frame,
    pub frames: Vec<Frame>,
}

impl FrameManager {
    pub fn current(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.main_frame)
    }

    pub fn current_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap_or(&mut self.main_frame)
    }

    pub fn push(&mut self, function: Function) {
        self.frames.push(Frame {
            function,
            ..Default::default()
        })
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}
