use crate::{
    ast::{self, Expression, Statement},
    error::ParserError,
    lexer,
    token::{arithmetic_tokens, assignment_tokens, bitwise_tokens, comparison_tokens, Token},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    AssignmentOps,
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equality,
    Relational,
    Shift,
    Additive,
    Multiplicative,
    Prefix,
    Call,
    Index,
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            assignment_tokens!() => Self::AssignmentOps,
            Token::Or => Self::LogicalOr,
            Token::And => Self::LogicalAnd,
            Token::BitOr => Self::BitOr,
            Token::BitXor => Self::BitXor,
            Token::BitAnd => Self::BitAnd,
            Token::Eq | Token::Ne => Self::Equality,
            Token::Lt | Token::Le | Token::Gt | Token::Ge => Self::Relational,
            Token::ShiftLeft | Token::ShiftRight => Self::Shift,
            Token::Add | Token::Sub => Self::Additive,
            Token::Div | Token::Mul | Token::Mod => Self::Multiplicative,
            Token::LeftParen => Self::Call,
            Token::LeftBracket => Self::Index,
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

        let mut program = ast::Program::default();

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

            // parse_if: parse if expression as statement
            Token::If => {
                let expression = self.parse_if()?;

                self.has_semicolon = optional_peek!(self, Token::Semicolon);

                Ok(Statement::Expression(ast::ExpressionStatement {
                    token: Token::If,
                    expression,
                }))
            }

            _ => {
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
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut left_expr = self.parse_prefix()?;

        while precedence < Precedence::from(&self.peek_token) {
            match self.parse_infix(&left_expr)? {
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

    pub fn parse_infix(&mut self, left: &Expression) -> Result<Option<Expression>, ParserError> {
        match self.peek_token {
            // parse_infix: parse infix expression
            arithmetic_tokens!()
            | comparison_tokens!()
            | bitwise_tokens!()
            | Token::Or
            | Token::And => {
                self.next_token()?;

                let token = self.curr_token.clone();
                let operator = self.curr_token.clone();
                let precedence = Precedence::from(&self.curr_token);

                self.next_token()?;

                let right = self.parse_expression(precedence)?;

                Ok(Some(Expression::Infix(ast::InfixExpression {
                    token,
                    left: Box::new(left.clone()),
                    operator,
                    right: Box::new(right),
                })))
            }

            // parse_call: parse call expression
            Token::LeftParen => {
                self.next_token()?;
                self.next_token()?;

                let mut args = Vec::new();

                if !matches!(self.curr_token, Token::RightParen) {
                    loop {
                        args.push(self.parse_expression(Precedence::Lowest)?);

                        if !matches!(self.peek_token, Token::Comma) {
                            break;
                        }

                        self.next_token()?;
                        self.next_token()?;
                    }

                    expect_peek!(self, Token::RightParen);
                }

                Ok(Some(Expression::Call(ast::CallExpression {
                    token: self.curr_token.clone(),
                    function: Box::new(left.clone()),
                    args,
                })))
            }

            Token::LeftBracket => {
                let token = self.curr_token.clone();

                self.next_token()?;
                self.next_token()?;

                let index = Box::new(self.parse_expression(Precedence::Lowest)?);

                expect_peek!(self, Token::RightBracket);

                Ok(Some(Expression::Index(ast::IndexExpression {
                    token,
                    left: Box::new(left.clone()),
                    index,
                })))
            }

            Token::ColonAssign | Token::Assign => {
                if !matches!(left, Expression::Identifier(_)) {
                    return Err(ParserError::InvalidLHS(left.clone()));
                }

                let name = ast::Identifier {
                    token: self.curr_token.clone(),
                    value: self.curr_token.to_string(),
                };

                self.next_token()?;
                let token = self.curr_token.clone();

                self.next_token()?;
                let value = Box::new(self.parse_expression(Precedence::Lowest)?);

                Ok(Some(Expression::Var(ast::VarExpression {
                    token,
                    name,
                    value,
                })))
            }

            Token::AddAssign
            | Token::SubAssign
            | Token::MulAssign
            | Token::DivAssign
            | Token::ModAssign
            | Token::ShiftLeftAssign
            | Token::ShiftRightAssign => {
                if !matches!(left, Expression::Identifier(_)) {
                    return Err(ParserError::InvalidLHS(left.clone()));
                }

                let name = ast::Identifier {
                    token: self.curr_token.clone(),
                    value: self.curr_token.to_string(),
                };

                self.next_token()?;
                let token = self.curr_token.clone();

                self.next_token()?;
                let value = self.parse_expression(Precedence::Lowest)?;

                // probably need to change this monstrosity.
                Ok(Some(Expression::Var(ast::VarExpression {
                    token: Token::Assign,
                    name: name.clone(),
                    value: Box::new(Expression::Infix(ast::InfixExpression {
                        left: Box::new(Expression::Identifier(name)),
                        operator: match &token {
                            Token::AddAssign => Token::Add,
                            Token::SubAssign => Token::Sub,
                            Token::MulAssign => Token::Mul,
                            Token::DivAssign => Token::Div,
                            Token::ModAssign => Token::Mod,
                            Token::ShiftLeftAssign => Token::ShiftLeft,
                            Token::ShiftRightAssign => Token::ShiftRight,
                            _ => unreachable!(),
                        },
                        token,
                        right: Box::new(value),
                    })),
                })))
            }

            _ => Ok(None),
        }
    }

    pub fn parse_prefix(&mut self) -> Result<Expression, ParserError> {
        match self.curr_token {
            // parse_identifier: parse current token as identifier
            Token::Ident(ref i) => Ok(Expression::Identifier(ast::Identifier {
                token: self.curr_token.clone(),
                value: i.into(),
            })),

            // parse_integer: parse current token as integer
            Token::Int(ref i) => match i.parse::<i64>() {
                Ok(lit) => Ok(Expression::Integer(ast::IntegerLiteral {
                    token: self.curr_token.clone(),
                    value: lit,
                })),
                Err(_) => Err(ParserError::ParsingInteger(i.into())),
            },

            // parse_float: parse current token as float
            Token::Float(ref f) => match f.parse::<f64>() {
                Ok(lit) => Ok(Expression::Float(ast::FloatLiteral {
                    token: self.curr_token.clone(),
                    value: lit,
                })),
                Err(_) => Err(ParserError::ParsingFloat(f.into())),
            },

            // parse_boolean: parse current token as boolean
            Token::True | Token::False => Ok(Expression::Boolean(ast::BooleanExpression {
                token: self.curr_token.clone(),
                value: matches!(self.curr_token, Token::True),
            })),

            // parse_string: parse current expression as string
            Token::String(ref s) => Ok(Expression::String(ast::StringLiteral {
                token: self.curr_token.clone(),
                value: s.into(),
            })),

            // parse_array
            Token::LeftBracket => Ok(Expression::Array(ast::ArrayLiteral {
                token: self.curr_token.clone(),
                elements: {
                    self.next_token()?;

                    let mut elements = Vec::new();

                    if !matches!(self.curr_token, Token::RightBracket) {
                        loop {
                            elements.push(self.parse_expression(Precedence::Lowest)?);

                            if !matches!(self.peek_token, Token::Comma) {
                                break;
                            }

                            self.next_token()?;
                            self.next_token()?;
                        }

                        expect_peek!(self, Token::RightBracket);
                    }

                    elements
                },
            })),

            // parse_prefix: parse current expression with prefix
            Token::Not | Token::Sub => {
                let prev_token = self.curr_token.clone();

                self.next_token()?;

                let right = self.parse_expression(Precedence::Prefix).unwrap();

                Ok(Expression::Prefix(ast::PrefixExpression {
                    operator: prev_token.clone(),
                    token: prev_token,
                    right: Box::new(right),
                }))
            }

            // parse_grouped: parse grouped expression
            Token::LeftParen => {
                self.next_token()?;
                let expr = self.parse_expression(Precedence::Lowest);

                expect_peek!(self, Token::RightParen);

                expr
            }

            // parse_block
            Token::LeftBrace => {
                let block = self.parse_block()?;
                Ok(Expression::Block(block))
            }

            // parse_if: parse current if expression
            Token::If => self.parse_if(),

            // parse_function: parse current expression as function
            Token::Function => {
                let token = self.curr_token.clone();
                let mut params = Vec::new();

                expect_peek!(self, Token::LeftParen);

                self.next_token()?;

                if !matches!(self.curr_token, Token::RightParen) {
                    params.push(ast::Identifier {
                        token: self.curr_token.clone(),
                        value: self.curr_token.to_string(),
                    });

                    while matches!(self.peek_token, Token::Comma) {
                        self.next_token()?;
                        self.next_token()?;

                        params.push(ast::Identifier {
                            token: self.curr_token.clone(),
                            value: self.curr_token.to_string(),
                        });
                    }

                    expect_peek!(self, Token::RightParen);
                }

                expect_peek!(self, Token::LeftBrace);

                let body = self.parse_block()?;

                Ok(Expression::Function(ast::FunctionLiteral {
                    token,
                    params,
                    body,
                }))
            }

            _ => Err(ParserError::UnknownPrefixOperator(self.curr_token.clone())),
        }
    }
}
