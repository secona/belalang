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

        f.write_str(&format!(
            "CallExpression(function={}, args=[{}])",
            self.function, args
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, testing};

    #[test]
    fn parsing() {
        let input = "add(1, 2 * 3, 4 + 5)".as_bytes().into();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement)
                .expect("not a(n) ast::Statement::ExpressionStatement");

        let expr = testing::as_variant!(&stmt.expression, ast::Expression::CallExpression)
            .expect("not a(n) ast::Expression::CallExpression");

        testing::expr!(&*expr.function, ast::Expression::Identifier = "add");

        assert_eq!(expr.args.len(), 3);
        testing::expr!(&expr.args[0], ast::Expression::IntegerLiteral = 1);
        testing::infix!(
            &expr.args[1],
            ast::Expression::IntegerLiteral = 2,
            "*",
            ast::Expression::IntegerLiteral = 3
        );
        testing::infix!(
            &expr.args[2],
            ast::Expression::IntegerLiteral = 4,
            "+",
            ast::Expression::IntegerLiteral = 5
        );
    }

    #[test]
    fn parsing_function_literal() {
        let input = "fn(x, y) { x + y }(2, 3)".as_bytes().into();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement)
                .expect("not a(n) ast::ExpressionStatement");

        let expr = testing::as_variant!(&stmt.expression, ast::Expression::CallExpression)
            .expect("not a(n) ast::Expression::CallExpression");

        assert_eq!(expr.args.len(), 2);
        testing::expr!(&expr.args[0], ast::Expression::IntegerLiteral = 2);
        testing::expr!(&expr.args[1], ast::Expression::IntegerLiteral = 3);

        let function = testing::as_variant!(&*expr.function, ast::Expression::FunctionLiteral)
            .expect("not a(n) ast::Expression::FunctionLiteral");

        assert_eq!(function.params.len(), 2);

        testing::ident!(function.params[0], "x");
        testing::ident!(function.params[1], "y");

        assert_eq!(function.body.statements.len(), 1);

        let body_stmt = testing::as_variant!(
            &function.body.statements[0],
            ast::Statement::ExpressionStatement
        )
        .expect("not a(n) ast::Statement::ExpressionStatement");

        testing::infix!(
            &body_stmt.expression,
            ast::Expression::Identifier = "x",
            "+",
            ast::Expression::Identifier = "y"
        );
    }
}
