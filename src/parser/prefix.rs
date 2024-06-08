use crate::{
    ast::{self, Expression},
    expect_peek, token,
};

use super::{error::ParserError, Precedence};

impl super::Parser<'_> {
    pub fn prefix_fn(&mut self) -> Result<Expression, ParserError> {
        match self.curr_token {
            token::Token::Ident(_) => self.parse_identifier(),
            token::Token::Int(_) => self.parse_integer_literal(),
            token::Token::Bang | token::Token::Minus => self.parse_prefix_expression(),
            token::Token::True | token::Token::False => self.parse_boolean(),
            token::Token::LParen => self.parse_grouped_expression(),
            token::Token::If => self.parse_if_expression(),
            token::Token::Function => self.parse_function_literal(),
            token::Token::String(_) => self.parse_string_literal(),
            _ => Err(ParserError::PrefixOperator(self.curr_token.clone())),
        }
    }

    fn parse_identifier(&self) -> Result<Expression, ParserError> {
        Ok(Expression::Identifier(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.clone().to_string(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, ParserError> {
        let literal = self.curr_token.clone().to_string();

        match literal.parse::<i64>() {
            Ok(lit) => Ok(Expression::IntegerLiteral(ast::IntegerLiteral {
                token: self.curr_token.clone(),
                value: lit,
            })),
            Err(_) => Err(ParserError::ParsingInteger(literal)),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let prev_token = self.curr_token.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix).unwrap();

        Ok(Expression::PrefixExpression(ast::PrefixExpression {
            operator: prev_token.clone(),
            token: prev_token,
            right: Box::new(right),
        }))
    }

    fn parse_boolean(&self) -> Result<Expression, ParserError> {
        Ok(Expression::BooleanExpression(ast::BooleanExpression {
            token: self.curr_token.clone(),
            value: matches!(self.curr_token, token::Token::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);

        expect_peek!(self, token::Token::RParen);

        expr
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        let token = self.curr_token.clone();

        expect_peek!(self, token::Token::LParen);

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest).unwrap();

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

        Ok(Expression::IfExpression(ast::IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.curr_token.clone();

        expect_peek!(self, token::Token::LParen);

        let params = self.parse_function_params()?;

        expect_peek!(self, token::Token::LBrace);

        let body = self.parse_block_statement();

        Ok(Expression::FunctionLiteral(ast::FunctionLiteral {
            token,
            params,
            body,
        }))
    }

    fn parse_string_literal(&mut self) -> Result<Expression, ParserError> {
        Ok(Expression::StringLiteral(ast::StringLiteral {
            token: self.curr_token.clone(),
            value: self.curr_token.to_string(),
        }))
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
