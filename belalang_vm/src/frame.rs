use std::vec::IntoIter;

use crate::object::{Function, Object};

#[derive(Debug, Default)]
pub struct Frame {
    pub function: Function,
    pub slots: Vec<Object>,
    pub ip: usize,
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

    pub fn append_to_main(&mut self, instructions: IntoIter<u8>) {
        self.main_frame.function.instructions.extend(instructions);
    }

    fn current(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.main_frame)
    }

    fn current_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap_or(&mut self.main_frame)
    }

    pub fn current_ip(&self) -> usize {
        self.current().ip
    }

    pub fn increment_ip(&mut self, value: usize) {
        self.current_mut().ip = self
            .current()
            .ip
            .checked_add_signed(value as isize)
            .unwrap();
    }

    pub fn current_inst(&self) -> &Vec<u8> {
        &self.current().function.instructions
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
