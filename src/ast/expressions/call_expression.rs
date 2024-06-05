use super::Expression;
use crate::token;

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

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("add(1, 2 * 3, 4 + 5)");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let expr = testing::as_variant!(&stmt.expression, ast::Expression::CallExpression);

        testing::expr_variant!(&*expr.function, ast::Expression::Identifier = "add");

        assert_eq!(expr.args.len(), 3);
        testing::expr_variant!(&expr.args[0], ast::Expression::IntegerLiteral = 1);
        testing::expr_variant!(
            &expr.args[1], Infix => (
                ast::Expression::IntegerLiteral = 2,
                token::Token::Asterisk,
                ast::Expression::IntegerLiteral = 3
            )
        );
        testing::expr_variant!(
            &expr.args[2], Infix => (
                ast::Expression::IntegerLiteral = 4,
                token::Token::Plus,
                ast::Expression::IntegerLiteral = 5
            )
        );
    }

    #[test]
    fn parsing_with_function_literal() {
        let program = testing::test_parse("fn(x, y) { x + y }(2, 3)");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let expr = testing::as_variant!(&stmt.expression, ast::Expression::CallExpression);

        assert_eq!(expr.args.len(), 2);
        testing::expr_variant!(&expr.args[0], ast::Expression::IntegerLiteral = 2);
        testing::expr_variant!(&expr.args[1], ast::Expression::IntegerLiteral = 3);

        let function = testing::as_variant!(&*expr.function, ast::Expression::FunctionLiteral);

        assert_eq!(function.params.len(), 2);

        testing::ident_has_name!(function.params[0], "x");
        testing::ident_has_name!(function.params[1], "y");

        assert_eq!(function.body.statements.len(), 1);

        let body_stmt = testing::as_variant!(
            &function.body.statements[0],
            ast::Statement::ExpressionStatement
        );

        testing::expr_variant!(
            &body_stmt.expression, Infix => (
                ast::Expression::Identifier = "x",
                token::Token::Plus,
                ast::Expression::Identifier = "y"
            )
        );
    }
}
