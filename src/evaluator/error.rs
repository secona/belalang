use super::object::Object;
use crate::token::Token;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    PrefixOperator(Token, Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(Object, Token, Object),

    #[error("identifier not found: {0}")]
    IdentifierNotFound(String),

    #[error("not a function")]
    NotAFunction(),

    #[error("overwriting builtin: {0}")]
    OverwriteBuiltin(String),

    #[error("variable redeclaration: {0}")]
    VariableRedeclaration(String),
}
