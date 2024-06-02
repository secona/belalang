mod environment;

pub use environment::Environment;

use crate::ast;

#[derive(Debug, Clone)]
pub enum Object<'a> {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object<'a>>),
    Error(String),

    Function {
        params: Vec<ast::Identifier>,
        body: ast::BlockStatement,
        env: Environment<'a>,
    }
}

impl std::fmt::Display for Object<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => f.write_str(&format!("{}", i)),
            Self::Boolean(b) => f.write_str(&format!("{}", b)),
            Self::Return(r) => f.write_str(&format!("{}", r)),
            Self::Null => f.write_str("null"),
            Self::Error(msg) => f.write_str(&msg),
            Self::Function { params, body, env } => f.write_str(""),
        }
    }
}

impl PartialEq for Object<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}
