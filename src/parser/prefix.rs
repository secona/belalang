use crate::{
    ast::{self, Expression},
    expect_peek, token,
};

use super::{error::ParserError, Precedence};

impl super::Parser<'_> {
    pub fn prefix_fn(&mut self) -> Result<Expression, ParserError> {
        match self.curr_token {
            // parse_identifier: parse current token as identifier
            token::Token::Ident(_) => Ok(Expression::Identifier(ast::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.clone().to_string(),
            })),

            // parse_integer: parse current token as integer
            token::Token::Int(_) => match self.curr_token.to_string().parse::<i64>() {
                Ok(lit) => Ok(Expression::Integer(ast::IntegerLiteral {
                    token: self.curr_token.clone(),
                    value: lit,
                })),
                Err(_) => Err(ParserError::ParsingInteger(self.curr_token.to_string())),
            },

            // parse_boolean: parse current token as boolean
            token::Token::True | token::Token::False => {
                Ok(Expression::Boolean(ast::BooleanExpression {
                    token: self.curr_token.clone(),
                    value: matches!(self.curr_token, token::Token::True),
                }))
            }

            // parse_string: parse current expression as string
            token::Token::String(_) => Ok(Expression::String(ast::StringLiteral {
                token: self.curr_token.clone(),
                value: self.curr_token.to_string(),
            })),

            // parse_prefix: parse current expression with prefix
            token::Token::Bang | token::Token::Minus => {
                let prev_token = self.curr_token.clone();

                self.next_token();

                let right = self.parse_expression(Precedence::Prefix).unwrap();

                Ok(Expression::Prefix(ast::PrefixExpression {
                    operator: prev_token.clone(),
                    token: prev_token,
                    right: Box::new(right),
                }))
            }

            // parse_grouped: parse grouped expression
            token::Token::LParen => {
                self.next_token();

                let expr = self.parse_expression(Precedence::Lowest);

                expect_peek!(self, token::Token::RParen);

                expr
            }

            // parse_if: parse current if expression
            token::Token::If => {
                let token = self.curr_token.clone();

                expect_peek!(self, token::Token::LParen);

                self.next_token();
                let condition = self.parse_expression(Precedence::Lowest)?;

                expect_peek!(self, token::Token::RParen);

                expect_peek!(self, token::Token::LBrace);

                let consequence = self.parse_block_statement();

                let alternative = if matches!(self.peek_token, token::Token::Else) {
                    self.next_token();

                    expect_peek!(self, token::Token::LBrace);

                    Some(self.parse_block_statement())
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

            // parse_function: parse current expression as function
            token::Token::Function => {
                let token = self.curr_token.clone();

                expect_peek!(self, token::Token::LParen);

                let params = self.parse_function_params()?;

                expect_peek!(self, token::Token::LBrace);

                let body = self.parse_block_statement();

                Ok(Expression::Function(ast::FunctionLiteral {
                    token,
                    params,
                    body,
                }))
            }

            _ => Err(ParserError::PrefixOperator(self.curr_token.clone())),
        }
    }

    fn parse_function_params(&mut self) -> Result<Vec<ast::Identifier>, ParserError> {
        let mut identifiers = Vec::new();

        if matches!(self.peek_token, token::Token::RParen) {
            self.next_token();
            return Ok(identifiers);
        }

        self.next_token();
        identifiers.push(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.to_string(),
        });

        while matches!(self.peek_token, token::Token::Comma) {
            self.next_token();
            self.next_token();

            identifiers.push(ast::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.to_string(),
            });
        }

        expect_peek!(self, token::Token::RParen);

        Ok(identifiers)
    }
}
