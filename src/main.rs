use repl::Repl;

pub mod ast;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

#[cfg(test)]
mod testing;

fn main() {
    Repl::start();
}
