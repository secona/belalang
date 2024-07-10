use belalang_comp::compiler::Compiler;
use belalang_comp::object::{Function, Object};

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

pub struct FrameManager {
    pub main_frame: Frame,
    pub frames: Vec<Frame>,
}

impl FrameManager {
    pub fn new(mut compiler: Compiler) -> Self {
        let instructions = compiler
            .scope
            .current_mut()
            .instructions
            .drain(..)
            .collect();

        Self {
            main_frame: Frame {
                function: Function {
                    instructions,
                    arity: 0,
                },
                slots: Vec::new(),
                ip: 0,
            },
            frames: Vec::new(),
        }
    }

    pub fn current(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.main_frame)
    }
}
