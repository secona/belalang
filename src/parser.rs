use crate::{ast, lexer, token};

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: Option<token::Token>,
    peek_token: Option<token::Token>,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            curr_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        if let Some(token) = &self.peek_token {
            self.curr_token = Some(token.clone());
        }

        let token = self.lexer.next_token();
        self.peek_token = Some(token);
    }

    fn curr_token_is(&self, other: token::Token) -> bool {
        matches!(&self.curr_token, Some(tok) if { *tok == other })
    }

    fn peek_token_is(&self, other: token::Token) -> bool {
        matches!(&self.peek_token, Some(tok) if { *tok == other })
    }

    fn expect_peek(&mut self, other: token::Token) -> bool {
        if self.peek_token_is(other) {
            self.next_token();
            return true;
        }

        false
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        while !self.curr_token_is(token::Token::EOF) {
            match self.parse_statement() {
                Some(stmt) => program.add_stmt(Box::new(stmt)),
                None => {}
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<impl ast::Statement> {
        match &self.curr_token {
            Some(tok) => match *tok {
                token::Token::Let => self.parse_let_statement(),
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        let let_token = self.curr_token.clone().unwrap();

        if !self.expect_peek(token::Token::Ident("".into())) {
            return None;
        }

        let name = ast::Identifier {
            token: self.curr_token.clone().unwrap(),
            value: String::from(""),
        };

        if !self.expect_peek(token::Token::Assign) {
            return None;
        }

        while !self.curr_token_is(token::Token::Semicolon) {
            self.next_token();
        }

        let stmt = ast::LetStatement {
            name,
            token: let_token,
            value: Box::new(ast::Identifier {
                value: "5".to_owned(),
                token: token::Token::Ident("5".to_owned()),
            }),
        };

        Some(stmt)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Node, lexer, token};

    #[test]
    fn let_statements() {
        let input = "let x = 5;".to_owned().into_bytes().into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let program = parser.parse_program();
        let tok = program.token();

        if let Some(inner) = tok {
            assert_eq!(*inner, token::Token::Let);
        }
    }
}
