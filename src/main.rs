use repl::Repl;

pub mod lexer;
pub mod token;
pub mod repl;

fn main() {
    Repl::start();
}
