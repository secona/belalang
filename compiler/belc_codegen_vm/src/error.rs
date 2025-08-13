use belc_lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum CodegenError {
    #[error("unknown infix operator: {0}")]
    UnknownInfixOp(Token),

    #[error("duplicate symbol: {0}")]
    DuplicateSymbol(String),

    #[error("unknown symbol: {0}")]
    UnknownSymbol(String),
}
