use belalang_core::ast::{BlockExpression, Expression, Program, Statement};
use belalang_core::token::Token;
use belalang_vm::builtins::BuiltinCollection;
use belalang_vm::bytecode::Bytecode;
use belalang_vm::object::{Function, Object};
use belalang_vm::opcode;

use crate::error::CompileError;
use crate::scope::{ScopeLevel, ScopeManager, ScopeManagerBuilder};

pub struct Compiler {
    incremental: bool,
    prev_constants: usize,

    pub constants: Vec<Object>,
    pub scope: ScopeManager,
}

impl Compiler {
    pub fn compile_program(&mut self, program: Program) -> Result<Bytecode, CompileError> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }

        let code = Bytecode {
            instructions: self.scope.main_scope.instructions.drain(..).collect(),
            constants: if self.incremental {
                let constants = self.constants[self.prev_constants..].to_vec();
                self.prev_constants = self.constants.len();
                constants
            } else {
                self.constants.drain(..).collect()
            },
        };

        Ok(code)
    }

    pub fn compile_statement(&mut self, statement: Statement) -> Result<(), CompileError> {
        match statement {
            Statement::Expression(statement) => {
                self.compile_expression(statement.expression)?;
                self.add_bytecode(opcode::POP);
            }

            Statement::Return(r#return) => {
                self.compile_expression(r#return.return_value)?;
                self.add_bytecode(opcode::RETURN_VALUE);
            }

            Statement::While(r#while) => {
                let start_of_while = self.scope.current().instructions.len();

                self.compile_expression(*r#while.condition)?;

                let jif = self.add_instruction(opcode::jump_if_false(0).to_vec());
                let jif_index = self.scope.current().instructions.len();

                // self.scope.enter();
                self.compile_block(r#while.block)?;
                // let scope = self.scope.leave();

                // self.scope
                //     .current_mut()
                //     .instructions
                //     .extend(scope.instructions);

                let jump = self.add_instruction(opcode::jump(0).to_vec());
                let current = self.scope.current().instructions.len();

                self.replace_u16_operand(jump, (start_of_while as isize - current as isize) as u16);
                self.replace_u16_operand(jif, (current as isize - jif_index as isize) as u16);

                self.add_bytecode(opcode::NULL);
                self.add_bytecode(opcode::POP);
            }
        };

        Ok(())
    }

    pub fn compile_expression(&mut self, expression: Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Boolean(boolean) => {
                self.add_bytecode(if boolean.value {
                    opcode::TRUE
                } else {
                    opcode::FALSE
                });
            }

            Expression::Integer(integer) => {
                let integer = Object::Integer(integer.value);
                let index = self.add_constant(integer) as u16;
                self.add_instruction(opcode::constant(index).to_vec());
            }

            Expression::Float(float) => {
                let float = Object::Float(float.value);
                let index = self.add_constant(float) as u16;
                self.add_instruction(opcode::constant(index).to_vec());
            }

            Expression::String(string) => {
                let string = Object::String(string.value);
                let index = self.add_constant(string) as u16;
                self.add_instruction(opcode::constant(index).to_vec());
            }

            Expression::Null(_) => {
                let null = Object::Null;
                let index = self.add_constant(null) as u16;
                self.add_instruction(opcode::constant(index).to_vec());
            }

            Expression::Array(_) => todo!(),

            Expression::Var(var) => match var.token {
                Token::ColonAssign => {
                    let symbol = self.scope.define(var.name.value)?;
                    let scope = symbol.scope;
                    let index = symbol.index;

                    self.compile_expression(*var.value)?;

                    self.set_variable(&scope, index);
                }
                Token::Assign
                | Token::AddAssign
                | Token::SubAssign
                | Token::MulAssign
                | Token::DivAssign
                | Token::ModAssign => {
                    let symbol = self.scope.resolve(var.name.value)?;
                    let scope = symbol.scope;
                    let index = symbol.index;

                    match var.token {
                        Token::Assign => {
                            self.compile_expression(*var.value)?;
                        }
                        Token::AddAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(opcode::ADD);
                        }
                        Token::SubAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(opcode::SUB);
                        }
                        Token::MulAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(opcode::MUL);
                        }
                        Token::DivAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(opcode::DIV);
                        }
                        Token::ModAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(opcode::MOD);
                        }
                        _ => unreachable!(),
                    }

                    self.set_variable(&scope, index);
                }
                _ => todo!(),
            },

            Expression::Call(call) => {
                for arg in call.args.into_iter().rev() {
                    self.compile_expression(arg)?;
                }

                self.compile_expression(*call.function)?;
                self.add_bytecode(opcode::CALL);
            }

            Expression::Index(_) => todo!(),

            Expression::Function(mut function) => {
                self.scope.enter();

                let arity = function.params.len();

                for param in function.params.drain(..) {
                    self.scope.define(param.value)?;
                }

                for statement in function.body.statements {
                    self.compile_statement(statement)?;
                }

                let scope = self.scope.leave();
                let mut instructions = scope.instructions;

                match instructions.last() {
                    Some(&opcode::RETURN_VALUE) => (),
                    _ => instructions.push(opcode::RETURN_VALUE),
                };

                let index = self.add_constant(Object::Function(Function {
                    arity,
                    instructions,
                })) as u16;

                self.add_instruction(opcode::constant(index).to_vec());
            }

            Expression::Identifier(ident) => {
                let symbol = self.scope.resolve(ident.value)?;
                let scope = symbol.scope;
                let index = symbol.index;

                self.get_variable(&scope, index);
            }

            Expression::If(r#if) => {
                self.compile_expression(*r#if.condition)?;

                let jif = self.add_instruction(opcode::jump_if_false(0).to_vec());
                let jif_index = self.scope.current().instructions.len();

                // self.scope.enter();
                self.compile_block(r#if.consequence)?;
                // let scope = self.scope.leave();

                // self.scope
                //     .current_mut()
                //     .instructions
                //     .extend(scope.instructions);

                let jump = self.add_instruction(opcode::jump(0).to_vec());
                let jump_index = self.scope.current().instructions.len();

                let post_consequence = self.scope.current().instructions.len();
                self.replace_u16_operand(jif, (post_consequence - jif_index) as u16);

                match r#if.alternative {
                    None => {
                        self.add_bytecode(opcode::NULL);
                    }
                    Some(alt) => match *alt {
                        Expression::Block(block) => {
                            // self.scope.enter();
                            self.compile_block(block)?;
                            // let scope = self.scope.leave();

                            // self.scope
                            //     .current_mut()
                            //     .instructions
                            //     .extend(scope.instructions);
                        }
                        _ => {
                            self.compile_expression(*alt)?;
                        }
                    },
                };

                let post_alternative = self.scope.current().instructions.len();
                self.replace_u16_operand(jump, (post_alternative - jump_index) as u16);
            }

            Expression::Infix(infix) => {
                match infix.operator {
                    Token::Gt | Token::Ge => {
                        self.compile_expression(*infix.right)?;
                        self.compile_expression(*infix.left)?;
                    }
                    _ => {
                        self.compile_expression(*infix.left)?;
                        self.compile_expression(*infix.right)?;
                    }
                }

                self.add_bytecode(match infix.operator {
                    Token::Add => opcode::ADD,
                    Token::Sub => opcode::SUB,
                    Token::Mul => opcode::MUL,
                    Token::Div => opcode::DIV,
                    Token::Mod => opcode::MOD,
                    Token::Eq => opcode::EQUAL,
                    Token::Ne => opcode::NOT_EQUAL,
                    Token::And => opcode::AND,
                    Token::Or => opcode::OR,
                    Token::BitAnd => opcode::BIT_AND,
                    Token::BitOr => opcode::BIT_OR,
                    Token::BitXor => opcode::BIT_XOR,
                    Token::ShiftLeft => opcode::BIT_SL,
                    Token::ShiftRight => opcode::BIT_SR,
                    Token::Lt | Token::Gt => opcode::LESS_THAN,
                    Token::Le | Token::Ge => opcode::LESS_THAN_EQUAL,
                    _ => return Err(CompileError::UnknownInfixOp(infix.operator)),
                });
            }

            Expression::Prefix(prefix) => {
                self.compile_expression(*prefix.right)?;
                self.add_bytecode(match prefix.operator {
                    Token::Sub => opcode::MINUS,
                    Token::Not => opcode::BANG,
                    _ => return Err(CompileError::UnknownInfixOp(prefix.operator)),
                });
            }

            Expression::Block(block) => {
                // self.scope.enter();
                self.compile_block(block)?;
                // let scope = self.scope.leave();

                // self.scope
                //     .current_mut()
                //     .instructions
                //     .extend(scope.instructions);
            }
        };

        Ok(())
    }

    fn compile_block(&mut self, block: BlockExpression) -> Result<(), CompileError> {
        for statement in block.statements {
            self.compile_statement(statement)?;
        }

        if let Some(&opcode::POP) = self.scope.current().instructions.last() {
            self.scope.current_mut().instructions.pop();
        }

        Ok(())
    }

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    pub fn add_bytecode(&mut self, byte: u8) -> usize {
        let scope = self.scope.current_mut();

        scope.instructions.push(byte);
        scope.instructions.len() - 1
    }

    pub fn add_instruction(&mut self, bytes: Vec<u8>) -> usize {
        let pos = self.scope.current_mut().instructions.len();

        for byte in bytes {
            self.add_bytecode(byte);
        }

        pos
    }

    pub fn replace_u16_operand(&mut self, index: usize, value: u16) {
        let scope = self.scope.current_mut();

        scope.instructions[index + 1] = (value >> 8) as u8;
        scope.instructions[index + 2] = (value & 0xFF) as u8;
    }

    fn get_variable(&mut self, scope: &ScopeLevel, index: usize) -> usize {
        match scope {
            ScopeLevel::Global => self.add_instruction(opcode::get_global(index as u16).to_vec()),
            ScopeLevel::Local => self.add_instruction(opcode::get_local(index as u8).to_vec()),
            ScopeLevel::Builtin => self.add_instruction(opcode::get_builtin(index as u8).to_vec()),
        }
    }

    fn set_variable(&mut self, scope: &ScopeLevel, index: usize) -> usize {
        match scope {
            ScopeLevel::Global => self.add_instruction(opcode::set_global(index as u16).to_vec()),
            ScopeLevel::Local => self.add_instruction(opcode::set_local(index as u8).to_vec()),
            ScopeLevel::Builtin => 0,
        }
    }
}

#[derive(Default)]
pub struct CompilerBuilder {
    incremental: bool,
    builtin_collection: Option<BuiltinCollection>,
}

impl CompilerBuilder {
    pub fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = incremental;
        self
    }

    pub fn builtin_collection(mut self, builtin_collection: BuiltinCollection) -> Self {
        self.builtin_collection = Some(builtin_collection);
        self
    }

    pub fn build(self) -> Compiler {
        let scope_manager = ScopeManagerBuilder::default()
            .builtin_collection(self.builtin_collection.unwrap_or_default())
            .build();

        Compiler {
            incremental: self.incremental,
            scope: scope_manager,
            constants: Vec::new(),
            prev_constants: 0,
        }
    }
}
