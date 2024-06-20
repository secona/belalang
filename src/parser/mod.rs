mod infix;
mod prefix;

use crate::{
    ast::{self, Expression, Statement},
    error::ParserError,
    lexer,
    token::Token,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    LogicalOr,
    LogicalAnd,
    Equality,
    Relational,
    Additive,
    Multiplicative,
    Prefix,
    Call,
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            Token::Or => Self::LogicalOr,
            Token::And => Self::LogicalAnd,
            Token::Eq | Token::Ne => Self::Equality,
            Token::Lt | Token::Le | Token::Gt | Token::Ge => Self::Relational,
            Token::Add | Token::Sub => Self::Additive,
            Token::Div | Token::Mul | Token::Mod => Self::Multiplicative,
            Token::LeftParen => Self::Call,
            _ => Self::Lowest,
        }
    }
}

macro_rules! expect_peek {
    ($self:expr, $token:pat) => {
        if matches!($self.peek_token, $token) {
            $self.next_token()?;
            true
        } else {
            return Err(ParserError::UnexpectedToken($self.peek_token.clone()));
        }
    };
}

pub(super) use expect_peek;

macro_rules! optional_peek {
    ($self:expr, $token:pat) => {
        if matches!($self.peek_token, $token) {
            $self.next_token()?;
            true
        } else {
            false
        }
    };
}

pub(super) use optional_peek;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    curr_token: Token,
    peek_token: Token,

    depth: i32,
    has_semicolon: bool,
}

impl Parser<'_> {
    pub fn new(lexer: lexer::Lexer<'_>) -> Parser {
        Parser {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),

            depth: 0,
            has_semicolon: false,
        }
    }

    fn next_token(&mut self) -> Result<(), ParserError> {
        self.curr_token = std::mem::take(&mut self.peek_token);
        self.peek_token = self.lexer.next_token()?;

        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParserError> {
        self.curr_token = self.lexer.next_token()?;
        self.peek_token = self.lexer.next_token()?;

        let mut program = ast::Program::new();

        while !matches!(self.curr_token, Token::EOF) {
            program.add_stmt(self.parse_statement()?);
            self.next_token()?;
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.curr_token {
            // parse_return
            Token::Return => {
                let token = self.curr_token.clone();

                self.next_token()?;
                let return_value = self.parse_expression(Precedence::Lowest)?;

                self.has_semicolon = expect_peek!(self, Token::Semicolon);

                Ok(Statement::Return(ast::ReturnStatement {
                    token,
                    return_value,
                }))
            }

            // parse_while
            Token::While => {
                let token = self.curr_token.clone();

                expect_peek!(self, Token::LeftParen);

                self.next_token()?;
                let condition = self.parse_expression(Precedence::Lowest)?;

                expect_peek!(self, Token::RightParen);

                expect_peek!(self, Token::LeftBrace);

                let block = self.parse_block()?;

                self.has_semicolon = optional_peek!(self, Token::Semicolon);

                Ok(Statement::While(ast::WhileStatement {
                    token,
                    condition: Box::new(condition),
                    block,
                }))
            }

            // parse_ident
            Token::Ident(_) => match self.peek_token {
                Token::ColonAssign | Token::Assign => {
                    let name = ast::Identifier {
                        token: self.curr_token.clone(),
                        value: self.curr_token.to_string(),
                    };

                    self.next_token()?;
                    let token = self.curr_token.clone();

                    self.next_token()?;
                    let value = self.parse_expression(Precedence::Lowest)?;

                    self.has_semicolon = expect_peek!(self, Token::Semicolon);

                    Ok(Statement::Var(ast::Var { token, name, value }))
                }
                Token::AddAssign
                | Token::SubAssign
                | Token::MulAssign
                | Token::DivAssign
                | Token::ModAssign => {
                    let name = ast::Identifier {
                        token: self.curr_token.clone(),
                        value: self.curr_token.to_string(),
                    };

                    self.next_token()?;
                    let token = self.curr_token.clone();

                    self.next_token()?;
                    let value = self.parse_expression(Precedence::Lowest)?;

                    self.has_semicolon = expect_peek!(self, Token::Semicolon);

                    // probably need to change this monstrosity.
                    Ok(Statement::Var(ast::Var {
                        token: Token::Assign,
                        name: name.clone(),
                        value: Expression::Infix(ast::InfixExpression {
                            left: Box::new(Expression::Identifier(name)),
                            operator: match &token {
                                Token::AddAssign => Token::Add,
                                Token::SubAssign => Token::Sub,
                                Token::MulAssign => Token::Mul,
                                Token::DivAssign => Token::Div,
                                Token::ModAssign => Token::Mod,
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
            Token::If => {
                let expression = self.parse_if()?;

                self.has_semicolon = optional_peek!(self, Token::Semicolon);

                Ok(Statement::Expression(ast::ExpressionStatement {
                    token: Token::If,
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
            expect_peek!(self, Token::Semicolon)
        } else {
            optional_peek!(self, Token::Semicolon)
        };

        Ok(Statement::Expression(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut left_expr = self.parse_prefix()?;

        while !matches!(self.peek_token, Token::Semicolon)
            && precedence < Precedence::from(&self.peek_token)
        {
            match self.parse_infix(&self.peek_token.clone(), &left_expr)? {
                Some(expr) => left_expr = expr,
                None => return Ok(left_expr),
            };
        }

        Ok(left_expr)
    }

    fn parse_block(&mut self) -> Result<ast::BlockExpression, ParserError> {
        let token = self.curr_token.clone();
        let mut statements = Vec::new();

        self.next_token()?;

        self.depth += 1;
        loop {
            if matches!(self.curr_token, Token::RightBrace | Token::EOF) {
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
            self.next_token()?;
        }
        self.depth -= 1;

        Ok(ast::BlockExpression { statements, token })
    }

    fn parse_if(&mut self) -> Result<Expression, ParserError> {
        let token = self.curr_token.clone();

        expect_peek!(self, Token::LeftParen);

        self.next_token()?;
        let condition = self.parse_expression(Precedence::Lowest)?;

        expect_peek!(self, Token::RightParen);

        expect_peek!(self, Token::LeftBrace);

        let consequence = self.parse_block()?;

        let alternative: Option<Box<Expression>> = if matches!(self.peek_token, Token::Else) {
            self.next_token()?;
            self.next_token()?;

            Some(Box::new(match self.curr_token {
                Token::If => self.parse_if()?,
                Token::LeftBrace => Expression::Block(self.parse_block()?),
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
