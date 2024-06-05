use crate::ast::expressions::Expression;
use crate::token;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.return_value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("return 12;");

        assert_eq!(program.statements.len(), 1);

        let ret = testing::as_variant!(&program.statements[0], ast::Statement::ReturnStatement);

        assert_eq!(ret.token, token::Token::Return);

        let val = testing::as_variant!(&ret.return_value, ast::Expression::IntegerLiteral);

        assert_eq!(val.token, token::Token::Int("12".into()));
        assert_eq!(val.value, 12);
    }
}
