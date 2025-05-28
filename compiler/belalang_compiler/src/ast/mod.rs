mod expressions;
mod parser;
mod program;
mod statements;

pub use expressions::*;
pub use parser::Parser;
pub use program::Program;
pub use statements::*;

pub enum Node {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}
