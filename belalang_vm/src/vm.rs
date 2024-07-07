use belalang_comp::code;
use belalang_comp::compiler::Compiler;
use belalang_comp::object::Object;

use crate::error::RuntimeError;

pub struct VM {
    pub constants: Vec<Object>,
    pub instructions: Vec<u8>,

    pub stack: Vec<Object>,
    pub sp: usize,

    pub last_popped: Object,
}

impl VM {
    pub fn new(compiler: Compiler) -> Self {
        Self {
            constants: compiler.constants,
            instructions: Vec::default(), // fix temporarily

            stack: Vec::new(),
            sp: 0,

            last_popped: Object::Integer(1), // TEMP
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let op = self.instructions[ip];

            match op {
                code::CONSTANT => {
                    let hi = self.instructions[ip + 1];
                    let lo = self.instructions[ip + 2];
                    let index = ((hi as u16) << 8) | (lo as u16);

                    let object = self.constants[index as usize].clone();
                    self.push(object)?;

                    ip += 2;
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

                code::EQ => todo!(),

                code::NE => todo!(),

                code::LT => todo!(),

                code::LE => todo!(),

                code::GT => todo!(),

                code::GE => todo!(),

                code::BANG => todo!(),

                code::MINUS => todo!(),

                _ => return Err(RuntimeError::UnknownInstruction(op)),
            };

            ip += 1;
        }

        Ok(())
    }

    pub fn stack_top(&mut self) -> Option<&Object> {
        if self.sp > 0 {
            self.stack.get(self.sp - 1)
        } else {
            None
        }
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
        compiler.compile_program(program).unwrap();

        let mut vm = VM::new(compiler);
        vm.run().unwrap();

        assert_eq!(vm.stack.len(), 0);

        let Object::Integer(v) = vm.last_popped else { panic!() };
        assert_eq!(v, 15);
    }
}
