use belalang_core::token::Token;

#[derive(thiserror::Error, Debug)]
pub enum CompileError {
    #[error("unknown infix operator: {0}")]
    UnknownInfixOp(Token)
}
