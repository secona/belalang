use belalang_comp::code;
use belalang_comp::compiler::Code;
use belalang_comp::object::Object;

use crate::error::RuntimeError;
use crate::frame::FrameManager;

#[derive(Default)]
pub struct VM {
    pub constants: Vec<Object>,
    pub globals: Vec<Object>,

    pub last_popped: Object,
    pub stack: Vec<Object>,
    pub sp: usize,

    pub frame: FrameManager,
}

impl VM {
    pub fn append_code(&mut self, code: &mut Code) {
        self.constants.append(&mut code.constants);
        self.frame
            .main_frame
            .function
            .instructions
            .append(&mut code.instructions);
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.frame.current().ip < self.frame.current().ins().len() {
            let op = self.frame.current().ins()[self.frame.current().ip];

            match op {
                code::CONSTANT => {
                    let index = self.read_u16();
                    let object = self.constants[index as usize].clone();
                    self.push(object)?;
                }

                code::POP => {
                    self.last_popped = self.pop()?;
                }

                code::ADD => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left + right))?;
                    };
                }

                code::SUB => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left - right))?;
                    };
                }

                code::MUL => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left * right))?;
                    };
                }

                code::DIV => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left / right))?;
                    };
                }

                code::MOD => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left % right))?;
                    };
                }

                code::TRUE => {
                    self.push(Object::Boolean(true))?;
                }

                code::FALSE => {
                    self.push(Object::Boolean(false))?;
                }

                code::EQUAL => todo!(),

                code::NOT_EQUAL => todo!(),

                code::LESS_THAN => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Boolean(left < right))?;
                    };
                }

                code::LESS_THAN_EQUAL => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Boolean(left <= right))?;
                    };
                }

                code::BANG => todo!(),

                code::MINUS => todo!(),

                code::JUMP => {
                    let relative = self.read_u16();
                    self.inc_ip(relative as usize);
                }

                code::JUMP_IF_FALSE => {
                    let relative = self.read_u16();
                    let value = self.pop()?;

                    if let Object::Boolean(false) = value {
                        self.inc_ip(relative as usize);
                    }
                }

                code::NULL => todo!(),

                // temporary
                code::DEF_GLOBAL | code::SET_GLOBAL => {
                    let index = self.read_u16() as usize;
                    let object = self.stack_top()?.clone();
                    self.globals.insert(index, object);
                }

                code::GET_GLOBAL => {
                    let index = self.read_u16() as usize;
                    self.push(self.globals[index].clone())?;
                }

                code::SET_LOCAL => {
                    let index = self.read_u8() as usize;
                    let object = self.stack_top()?.clone();
                    self.frame.current_mut().slots.insert(index, object);
                }

                code::GET_LOCAL => {
                    let index = self.read_u8() as usize;
                    self.push(self.frame.current().slots[index].clone())?;
                }

                code::CALL => {
                    if let Object::Function(function) = self.pop()? {
                        self.frame.push(function);
                        continue; // continue because we dont want to increment the ip
                    } else {
                        return Err(RuntimeError::NotAFunction);
                    }
                }

                code::RETURN => todo!(),

                code::RETURN_VALUE => {
                    if self.stack_top().is_err() {
                        self.push(Object::Null)?;
                    }

                    self.frame.pop();
                }

                _ => return Err(RuntimeError::UnknownInstruction(op)),
            };

            self.inc_ip(1)
        }

        Ok(())
    }

    pub fn inc_ip(&mut self, value: usize) {
        self.frame.current_mut().ip += value;
    }

    pub fn read_u16(&mut self) -> u16 {
        let current_frame = &mut self.frame.current_mut();

        let hi = current_frame.ins()[current_frame.ip + 1];
        let lo = current_frame.ins()[current_frame.ip + 2];
        current_frame.ip += 2;

        ((hi as u16) << 8) | (lo as u16)
    }

    pub fn read_u8(&mut self) -> u8 {
        let current_frame = &mut self.frame.current_mut();

        current_frame.ip += 1;
        current_frame.ins()[current_frame.ip]
    }

    pub fn stack_top(&mut self) -> Result<&Object, RuntimeError> {
        if self.sp == 0 {
            return Err(RuntimeError::StackUnderflow);
        }

        self.stack
            .get(self.sp - 1)
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn pop(&mut self) -> Result<Object, RuntimeError> {
        if self.sp == 0 {
            return Err(RuntimeError::StackUnderflow);
        }

        self.sp -= 1;

        Ok(self.stack.remove(self.sp))
    }

    fn push(&mut self, object: Object) -> Result<(), RuntimeError> {
        self.stack.push(object);
        self.sp += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use belalang_comp::{compiler::Compiler, object::Object};
    use belalang_core::{lexer::Lexer, parser::Parser};

    use super::VM;

    #[test]
    fn constant() {
        let lexer = Lexer::new("5 + 10;".as_bytes());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        let mut compiler = Compiler::default();
        let mut code = compiler.compile_program(program).unwrap();

        let mut vm = VM::default();
        vm.append_code(&mut code);
        vm.run().unwrap();

        assert_eq!(vm.stack.len(), 0);

        let Object::Integer(v) = vm.last_popped else {
            panic!()
        };
        assert_eq!(v, 15);
    }
}
