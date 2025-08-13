#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error(transparent)]
    Lexer(#[from] belc_lexer::LexerError),
}
