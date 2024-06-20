use crate::token;

use super::Statement;

#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub token: token::Token,
    pub value: bool,
}

impl std::fmt::Display for BooleanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: token::Token,
    pub value: String,
}

impl std::fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct NullLiteral {
    pub token: token::Token,
}

impl std::fmt::Display for NullLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

#[derive(Debug, Clone)]
pub struct VarExpression {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl std::fmt::Display for VarExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {};", self.name, self.token, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: token::Token,
    pub function: Box<Expression>,
    pub args: Vec<Expression>,
}

impl std::fmt::Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .args
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}({})", self.function, args)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub params: Vec<Identifier>,
    pub body: BlockExpression,
}

impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .params
            .iter()
            .map(|param| param.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "fn({}) {}", params, self.body)
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub consequence: BlockExpression,
    pub alternative: Option<Box<Expression>>,
}

impl std::fmt::Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if ({}) {} else {}",
            self.condition,
            self.consequence,
            match &self.alternative {
                Some(alt) => alt.to_string(),
                None => "{}".into(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub operator: token::Token,
    pub right: Box<Expression>,
}

impl std::fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: token::Token,
    pub right: Box<Expression>,
}

impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct BlockExpression {
    pub token: token::Token,
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for BlockExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{{ {} }}", statements)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Boolean(BooleanExpression),
    Integer(IntegerLiteral),
    String(StringLiteral),
    Null(NullLiteral),
    Var(VarExpression),
    Call(CallExpression),
    Function(FunctionLiteral),
    Identifier(Identifier),
    If(IfExpression),
    Infix(InfixExpression),
    Prefix(PrefixExpression),
    Block(BlockExpression),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Expression::Boolean(v) => v.to_string(),
            Expression::Integer(v) => v.to_string(),
            Expression::String(v) => v.to_string(),
            Expression::Null(v) => v.to_string(),
            Expression::Var(v) => v.to_string(),
            Expression::Call(v) => v.to_string(),
            Expression::Function(v) => v.to_string(),
            Expression::Identifier(v) => v.to_string(),
            Expression::If(v) => v.to_string(),
            Expression::Infix(v) => v.to_string(),
            Expression::Prefix(v) => v.to_string(),
            Expression::Block(v) => v.to_string(),
        })
    }
}
