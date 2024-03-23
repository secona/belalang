use crate::token::Token;

pub struct Lexer {
    input: &'static [u8],
    position: usize,
    read_position: usize,
    ch: Option<&'static u8>,
}

impl Lexer {
    pub fn new(input: &'static [u8]) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(&self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token::EMPTY;

        match self.ch {
            None => tok = Token::EOF(),
            Some(ch) => match ch {
                b'=' => tok = Token::ASSIGN(ch),
                b';' => tok = Token::SEMICOLON(ch),
                b'(' => tok = Token::LPAREN(ch),
                b')' => tok = Token::RPAREN(ch),
                b',' => tok = Token::COMMA(ch),
                b'+' => tok = Token::PLUS(ch),
                b'{' => tok = Token::LBRACE(ch),
                b'}' => tok = Token::RBRACE(ch),
                _ => {}
            },
        }

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn it_works() {
        let input = b"=+(){},;";

        let expected: [Token; 8] = [
            Token::ASSIGN(&b'='),
            Token::PLUS(&b'+'),
            Token::LPAREN(&b'('),
            Token::RPAREN(&b')'),
            Token::LBRACE(&b'{'),
            Token::RBRACE(&b'}'),
            Token::COMMA(&b','),
            Token::SEMICOLON(&b';'),
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            assert!(tok == exp);
        }
    }
}
