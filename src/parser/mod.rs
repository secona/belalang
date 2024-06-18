mod infix;
mod prefix;

use crate::{
    ast::{self, Expression, Statement},
    error::ParserError,
    lexer, token,
};

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
            token::Token::Eq | token::Token::Ne => Self::Equals,
            token::Token::Lt | token::Token::Le | token::Token::Gt | token::Token::Ge => {
                Self::LessGreater
            }
            token::Token::Add | token::Token::Sub => Self::Sum,
            token::Token::Div | token::Token::Mul | token::Token::Mod => Self::Product,
            token::Token::LeftParen => Self::Call,
            _ => Self::Lowest,
        }
    }
}

#[macro_export]
macro_rules! expect_peek {
    ($self:expr, $token:pat) => {
        if matches!($self.peek_token, $token) {
            $self.next_token();
            true
        } else {
            return Err(ParserError::UnexpectedToken($self.peek_token.clone()));
        }
    };
}

#[macro_export]
macro_rules! optional_peek {
    ($self:expr, $token:pat) => {
        if matches!($self.peek_token, $token) {
            $self.next_token();
            true
        } else {
            false
        }
    };
}

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    curr_token: token::Token,
    peek_token: token::Token,

    depth: i32,
    has_semicolon: bool,
}

impl Parser<'_> {
    pub fn new(mut lexer: lexer::Lexer<'_>) -> Parser {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            curr_token,
            peek_token,

            depth: 0,
            has_semicolon: false,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = std::mem::take(&mut self.peek_token);
        self.peek_token = self.lexer.next_token();
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
            // parse_return
            token::Token::Return => {
                let token = self.curr_token.clone();

                self.next_token();
                let return_value = self.parse_expression(Precedence::Lowest)?;

                self.has_semicolon = expect_peek!(self, token::Token::Semicolon);

                Ok(Statement::Return(ast::ReturnStatement {
                    token,
                    return_value,
                }))
            }

            // parse_while
            token::Token::While => {
                let token = self.curr_token.clone();

                expect_peek!(self, token::Token::LeftParen);

                self.next_token();
                let condition = self.parse_expression(Precedence::Lowest).unwrap();

                expect_peek!(self, token::Token::RightParen);

                expect_peek!(self, token::Token::LeftBrace);

                let block = self.parse_block()?;

                self.has_semicolon = optional_peek!(self, token::Token::Semicolon);

                Ok(Statement::While(ast::WhileStatement {
                    token,
                    condition: Box::new(condition),
                    block,
                }))
            }

            // parse_ident
            token::Token::Ident(_) => match self.peek_token {
                token::Token::ColonAssign | token::Token::Assign => {
                    let name = ast::Identifier {
                        token: self.curr_token.clone(),
                        value: self.curr_token.to_string(),
                    };

                    self.next_token();
                    let token = self.curr_token.clone();

                    self.next_token();
                    let value = self.parse_expression(Precedence::Lowest)?;

                    self.has_semicolon = expect_peek!(self, token::Token::Semicolon);

                    Ok(Statement::Var(ast::Var { token, name, value }))
                }
                token::Token::AddAssign
                | token::Token::SubAssign
                | token::Token::MulAssign
                | token::Token::DivAssign
                | token::Token::ModAssign => {
                    let name = ast::Identifier {
                        token: self.curr_token.clone(),
                        value: self.curr_token.to_string(),
                    };

                    self.next_token();
                    let token = self.curr_token.clone();

                    self.next_token();
                    let value = self.parse_expression(Precedence::Lowest)?;

                    self.has_semicolon = expect_peek!(self, token::Token::Semicolon);

                    // probably need to change this monstrosity.
                    Ok(Statement::Var(ast::Var {
                        token: token::Token::Assign,
                        name: name.clone(),
                        value: Expression::Infix(ast::InfixExpression {
                            left: Box::new(Expression::Identifier(name)),
                            operator: match &token {
                                token::Token::AddAssign => token::Token::Add,
                                token::Token::SubAssign => token::Token::Sub,
                                token::Token::MulAssign => token::Token::Mul,
                                token::Token::DivAssign => token::Token::Div,
                                token::Token::ModAssign => token::Token::Mod,
                                _ => unreachable!(),
                            },
                            token,
                            right: Box::new(value),
                        }),
                    }))
                }
                _ => self.parse_expression_statement(),
            },

            // parse_if: parse if expression as statement
            token::Token::If => {
                let expression = self.parse_if()?;

                self.has_semicolon = optional_peek!(self, token::Token::Semicolon);

                Ok(Statement::Expression(ast::ExpressionStatement {
                    token: token::Token::If,
                    expression,
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

        self.has_semicolon = if self.depth == 0 {
            expect_peek!(self, token::Token::Semicolon)
        } else {
            optional_peek!(self, token::Token::Semicolon)
        };

        Ok(Statement::Expression(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let prefix = self.prefix_fn()?;

        let mut left_expr: Result<Expression, Expression> = Ok(prefix);

        while !matches!(self.peek_token, token::Token::Semicolon)
            && precedence < Precedence::from(&self.peek_token)
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

    fn parse_block(&mut self) -> Result<ast::BlockExpression, ParserError> {
        let token = self.curr_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        self.depth += 1;
        loop {
            if matches!(
                self.curr_token,
                token::Token::RightBrace | token::Token::EOF
            ) {
                if let Some(Statement::Expression(_)) = statements.last() {
                    if !self.has_semicolon {
                        break;
                    }
                }

                statements.push(Statement::Expression(ast::ExpressionStatement {
                    token: self.curr_token.clone(),
                    expression: Expression::Null(ast::NullLiteral {
                        token: self.curr_token.clone(),
                    }),
                }));

                break;
            }

            statements.push(self.parse_statement()?);
            self.next_token();
        }
        self.depth -= 1;

        Ok(ast::BlockExpression { statements, token })
    }

    fn parse_if(&mut self) -> Result<Expression, ParserError> {
        let token = self.curr_token.clone();

        expect_peek!(self, token::Token::LeftParen);

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        expect_peek!(self, token::Token::RightParen);

        expect_peek!(self, token::Token::LeftBrace);

        let consequence = self.parse_block()?;

        let alternative: Option<Box<Expression>> = if matches!(self.peek_token, token::Token::Else)
        {
            self.next_token();
            self.next_token();

            Some(Box::new(match self.curr_token {
                token::Token::If => self.parse_if()?,
                token::Token::LeftBrace => Expression::Block(self.parse_block()?),
                _ => return Err(ParserError::UnexpectedToken(self.curr_token.clone())),
            }))
        } else {
            None
        };

        Ok(Expression::If(ast::IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }
}
