use belalang_core::ast::{Expression, Node, Program, Statement};

use crate::{
    code::{self, ToBytecode},
    error::CompileError,
    object::Object,
};

pub struct Compiler {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}

impl Compiler {
    pub fn compile(&mut self, node: Node) -> Result<(), CompileError> {
        match node {
            Node::Program(program) => self.compile_program(program),
            Node::Expression(expression) => self.compile_expression(expression),
            Node::Statement(statement) => self.compile_statement(statement),
        }
    }

    pub fn compile_program(&mut self, program: Program) -> Result<(), CompileError> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    pub fn compile_statement(&mut self, statement: Statement) -> Result<(), CompileError> {
        match statement {
            Statement::Expression(statement) => {
                self.compile_expression(statement.expression)?;
                self.add_bytecode(code::POP);
            }

            Statement::Return(_) => todo!(),
            Statement::While(_) => todo!(),
        };

        Ok(())
    }

    pub fn compile_expression(&mut self, expression: Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Boolean(_) => todo!(),

            Expression::Integer(integer) => {
                let integer = Object::Integer(integer.value);
                let index = self.add_constant(integer) as u16;
                self.add_instruction(code::constant(index).to_vec());
            }

            Expression::Float(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Null(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Var(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::Function(_) => todo!(),
            Expression::Identifier(_) => todo!(),
            Expression::If(_) => todo!(),

            Expression::Infix(infix) => {
                self.compile_expression(*infix.left)?;
                self.compile_expression(*infix.right)?;
                self.add_bytecode(infix.operator.to_bytecode()?)
            }

            Expression::Prefix(_) => todo!(),
            Expression::Block(_) => todo!(),
        };

        Ok(())
    }

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    pub fn add_bytecode(&mut self, byte: u8) {
        self.instructions.push(byte);
    }

    pub fn add_instruction(&mut self, bytes: Vec<u8>) -> usize {
        let pos = self.instructions.len();

        for byte in bytes {
            self.add_bytecode(byte);
        }

        pos
    }
}
