mod expressions;
mod parser;
mod program;
mod statements;

pub use expressions::*;
pub use statements::*;
pub use program::Program;
pub use parser::Parser;

pub enum Node {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}
