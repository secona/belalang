use crate::object::Object;
use belalang_core::token::Token;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    UnknownPrefixOperator(Token, Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(Object, Token, Object),

    #[error("unknown variable: {0}")]
    UnknownVariable(String),

    #[error("not a function")]
    NotAFunction,

    #[error("not an array")]
    NotAnArray,

    #[error("overwriting builtin: {0}")]
    OverwriteBuiltin(String),

    #[error("variable redeclaration: {0}")]
    VariableRedeclaration(String),

    #[error("illegal returning value: {0}")]
    ReturningValue(Object),
}
