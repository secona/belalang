use repl::Repl;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

#[cfg(test)]
mod test_util;

fn main() {
    Repl::start();
}
