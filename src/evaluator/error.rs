use crate::object;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {operator}{right}")]
    UnknownPrefixOperator {
        operator: String,
        right: object::Object,
    },
    #[error("unknown operator: {left} {operator} {right}")]
    UnknownInfixOperator {
        left: object::Object,
        operator: String,
        right: object::Object,
    },
    #[error("unknown operator: {left} {operator} {right}")]
    UnknownInfixOperatorInt {
        left: i64,
        operator: String,
        right: i64,
    },
    #[error("identifier not found: {name}")]
    IdentifierNotFound {
        name: String,
    }
}
