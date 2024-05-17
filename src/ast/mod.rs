mod expressions;
mod program;
mod statements;

pub use expressions::*;
pub use program::Program;
pub use statements::*;

pub enum Node {
    Expression(Expression),
    Statement(Statement),
    Program(Program),
}
