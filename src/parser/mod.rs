mod error;
mod infix;
mod prefix;

use crate::{
    ast::{self, Expression, Statement},
    lexer, token,
};

use self::error::ParserError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl From<&token::Token> for Precedence {
    fn from(value: &token::Token) -> Self {
        match value {
            token::Token::Eq | token::Token::NotEq => Self::Equals,
            token::Token::LT | token::Token::GT => Self::LessGreater,
            token::Token::Plus | token::Token::Minus => Self::Sum,
            token::Token::Slash | token::Token::Asterisk | token::Token::Percent => Self::Product,
            token::Token::LParen => Self::Call,
            _ => Self::Lowest,
        }
    }
}

#[macro_export]
macro_rules! expect_peek {
    ($self:expr, $token:pat) => {
        if matches!($self.peek_token, $token) {
            $self.next_token();
        } else {
            return Err(ParserError::UnexpectedToken($self.peek_token.clone()))
        }
    };
}

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    curr_token: token::Token,
    peek_token: token::Token,
}

impl Parser<'_> {
    pub fn new(mut lexer: lexer::Lexer<'_>) -> Parser {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            curr_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = std::mem::take(&mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn curr_precedence(&self) -> Precedence {
        Precedence::from(&self.curr_token)
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from(&self.peek_token)
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<ParserError>> {
        let mut program = ast::Program::new();
        let mut errors = Vec::<ParserError>::new();

        while !matches!(self.curr_token, token::Token::EOF) {
            match self.parse_statement() {
                Ok(stmt) => program.add_stmt(stmt),
                Err(err) => errors.push(err),
            }

            self.next_token();
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(program)
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.curr_token {
            token::Token::Return => self.parse_return_statement(),
            token::Token::Ident(_) => self.parse_ident(),
            token::Token::While => self.parse_while_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.curr_token.clone();

        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest)?;

        if matches!(self.peek_token, token::Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::ReturnStatement(ast::ReturnStatement {
            token,
            return_value,
        }))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.curr_token.clone();

        expect_peek!(self, token::Token::LParen);

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest).unwrap();

        expect_peek!(self, token::Token::RParen);

        expect_peek!(self, token::Token::LBrace);

        let block = self.parse_block_statement();

        Ok(Statement::WhileStatement(ast::WhileStatement {
            token,
            condition: Box::new(condition),
            block,
        }))
    }

    fn parse_ident(&mut self) -> Result<Statement, ParserError> {
        match self.peek_token {
            token::Token::Walrus | token::Token::Assign => {
                let name = ast::Identifier {
                    token: self.curr_token.clone(),
                    value: self.curr_token.to_string(),
                };

                self.next_token();
                let token = self.curr_token.clone();

                self.next_token();
                let value = self.parse_expression(Precedence::Lowest)?;

                if matches!(self.peek_token, token::Token::Semicolon) {
                    self.next_token();
                }

                Ok(Statement::Var(ast::Var {
                    token,
                    name,
                    value,
                }))
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let stmt = ast::ExpressionStatement {
            token: self.curr_token.clone(),
            expression: self.parse_expression(Precedence::Lowest)?,
        };

        if matches!(self.peek_token, token::Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let prefix = self.prefix_fn()?;

        let mut left_expr: Result<Expression, Expression> = Ok(prefix);

        while !matches!(self.peek_token, token::Token::Semicolon)
            && precedence < self.peek_precedence()
        {
            if let Ok(expr) = left_expr {
                left_expr = self.infix_fn(&self.peek_token.clone(), expr);
            }

            if let Err(expr) = left_expr {
                left_expr = Ok(expr);
                break;
            }
        }

        match left_expr {
            Ok(expr) => Ok(expr),
            Err(_) => Err(ParserError::PrefixOperator(token::Token::Function)),
        }
    }

    fn parse_block_statement(&mut self) -> ast::BlockStatement {
        let token = self.curr_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !matches!(self.curr_token, token::Token::RBrace | token::Token::EOF) {
            if let Ok(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        ast::BlockStatement { statements, token }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, testing, token::{self, Token}};

    #[test]
    fn integer_literal_expression() {
        let input = b"5;";

        let lexer = lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let literal = testing::as_variant!(&stmt.expression, ast::Expression::IntegerLiteral);

        assert_eq!(literal.value, 5);
        assert_eq!(literal.token, token::Token::Int("5".to_owned()));
    }

    struct PrefixTest {
        input: &'static [u8],
        exp_operator: Token,
        exp_right: String,
    }

    #[test]
    fn test_prefix_parsing() {
        let tests: [PrefixTest; 4] = [
            PrefixTest {
                input: b"!5;",
                exp_operator: Token::Bang,
                exp_right: String::from("5"),
            },
            PrefixTest {
                input: b"-15;",
                exp_operator: Token::Minus,
                exp_right: String::from("15"),
            },
            PrefixTest {
                input: b"!true;",
                exp_operator: Token::Bang,
                exp_right: String::from("true"),
            },
            PrefixTest {
                input: b"!false;",
                exp_operator: Token::Bang,
                exp_right: String::from("false"),
            },
        ];

        for test in tests {
            let lexer = lexer::Lexer::new(test.input);
            let mut parser = super::Parser::new(lexer);

            let program = parser.parse_program().expect("got parser errors");

            let stmt =
                testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

            let exp = testing::as_variant!(&stmt.expression, ast::Expression::PrefixExpression);

            assert_eq!(exp.operator, test.exp_operator);
            assert_eq!((*exp.right).to_string(), test.exp_right);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests: [[&str; 2]; 25] = [
            ["a * b + c", "((a * b) + c);"],
            ["!-a", "(!(-a));"],
            ["a + b + c", "((a + b) + c);"],
            ["a + b - c", "((a + b) - c);"],
            ["a * b * c", "((a * b) * c);"],
            ["a * b / c", "((a * b) / c);"],
            ["a + b / c", "(a + (b / c));"],
            ["a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f);"],
            ["3 + 4; -5 * 5", "(3 + 4);((-5) * 5);"],
            ["5 > 4 == 3 < 4", "((5 > 4) == (3 < 4));"],
            ["5 < 4 != 3 > 4", "((5 < 4) != (3 > 4));"],
            [
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));",
            ],
            [
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));",
            ],
            ["true", "true;"],
            ["false", "false;"],
            ["3 > 5 == false", "((3 > 5) == false);"],
            ["3 < 5 == true", "((3 < 5) == true);"],
            ["1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4);"],
            ["(5 + 5) * 2", "((5 + 5) * 2);"],
            ["2 / (5 + 5)", "(2 / (5 + 5));"],
            ["-(5 + 5)", "(-(5 + 5));"],
            ["!(true == true)", "(!(true == true));"],
            ["a + add(b * c) + d", "((a + add((b * c))) + d);"],
            [
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)));",
            ],
            [
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g));",
            ],
        ];

        for test in tests {
            let input = test[0].to_owned().into_bytes().into_boxed_slice();
            let lexer = lexer::Lexer::new(&input);
            let mut parser = super::Parser::new(lexer);

            let program = parser.parse_program().expect("got parser errors");
            assert_eq!(program.to_string(), test[1]);
        }
    }
}
