use super::object::Object;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    PrefixOperator(String, Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(Object, String, Object),

    #[error("identifier not found: {0}")]
    IdentifierNotFound(String),

    #[error("not a function")]
    NotAFunction(),
}
