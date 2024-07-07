use belalang_core::ast::{BlockExpression, Expression, Node, Program, Statement};
use belalang_core::token::Token;

use crate::code;
use crate::error::CompileError;
use crate::object::Object;
use crate::symbol_table::SymbolTable;

#[derive(Default)]
pub struct Compiler {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
    pub symbol_table: SymbolTable,
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
            Expression::Boolean(boolean) => {
                self.add_bytecode(if boolean.value {
                    code::TRUE
                } else {
                    code::FALSE
                });
            }

            Expression::Integer(integer) => {
                let integer = Object::Integer(integer.value);
                let index = self.add_constant(integer) as u16;
                self.add_instruction(code::constant(index).to_vec());
            }

            Expression::Float(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Null(_) => todo!(),
            Expression::Array(_) => todo!(),

            Expression::Var(var) => match var.token {
                Token::ColonAssign => {
                    self.compile_expression(*var.value)?;

                    let symbol = self.symbol_table.define(var.name.value)?;
                    let index = symbol.index;

                    self.add_instruction(code::set_global(index as u16).to_vec());
                }
                _ => todo!(),
            },

            Expression::Call(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::Function(_) => todo!(),

            Expression::Identifier(ident) => {
                let symbol = self.symbol_table.resolve(ident.value)?;
                self.add_instruction(code::get_global(symbol.index as u16).to_vec());
            }

            Expression::If(r#if) => {
                self.compile_expression(*r#if.condition)?;

                let jif = self.add_instruction(code::jump_if_false(0).to_vec());

                self.compile_block(r#if.consequence)?;
                if let Some(&code::POP) = self.instructions.last() {
                    self.instructions.pop();
                }

                let jump = self.add_instruction(code::jump(0).to_vec());

                let post_consequencee = self.instructions.len();
                self.replace_u16_operand(jif, post_consequencee as u16);

                match r#if.alternative {
                    None => {
                        self.add_bytecode(code::NULL);
                    }
                    Some(alternative) => {
                        self.compile_expression(*alternative)?;
                        if let Some(&code::POP) = self.instructions.last() {
                            self.instructions.pop();
                        }
                    }
                };

                let post_alternative = self.instructions.len();
                self.replace_u16_operand(jump, post_alternative as u16);
            }

            Expression::Infix(infix) => {
                self.compile_expression(*infix.left)?;
                self.compile_expression(*infix.right)?;
                self.add_bytecode(match infix.operator {
                    Token::Add => code::ADD,
                    Token::Sub => code::SUB,
                    Token::Mul => code::MUL,
                    Token::Div => code::DIV,
                    Token::Mod => code::MOD,
                    Token::Eq => code::EQ,
                    Token::Ne => code::NE,
                    Token::Lt => code::LT,
                    Token::Le => code::LE,
                    Token::Gt => code::GT,
                    Token::Ge => code::GE,
                    _ => return Err(CompileError::UnknownInfixOp(infix.operator)),
                });
            }

            Expression::Prefix(prefix) => {
                self.compile_expression(*prefix.right)?;
                self.add_bytecode(match prefix.operator {
                    Token::Sub => code::MINUS,
                    Token::Not => code::BANG,
                    _ => return Err(CompileError::UnknownInfixOp(prefix.operator)),
                });
            }

            Expression::Block(block) => {
                self.compile_block(block)?;
            }
        };

        Ok(())
    }

    fn compile_block(&mut self, block: BlockExpression) -> Result<(), CompileError> {
        for statement in block.statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    pub fn add_bytecode(&mut self, byte: u8) -> usize {
        self.instructions.push(byte);
        self.instructions.len() - 1
    }

    pub fn add_instruction(&mut self, bytes: Vec<u8>) -> usize {
        let pos = self.instructions.len();

        for byte in bytes {
            self.add_bytecode(byte);
        }

        pos
    }

    pub fn replace_u16_operand(&mut self, index: usize, value: u16) {
        self.instructions[index + 1] = (value >> 8) as u8;
        self.instructions[index + 2] = (value & 0xFF) as u8;
    }
}