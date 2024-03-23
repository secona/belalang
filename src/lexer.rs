use crate::token::Token;

pub struct Lexer {
    input: Box<[u8]>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &[u8]) -> Lexer {
        let mut lexer = Lexer {
            input: input.into(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token::EMPTY;

        match self.ch {
            b'=' => tok = Token::ASSIGN(self.ch),
            b';' => tok = Token::SEMICOLON(self.ch),
            b'(' => tok = Token::LPAREN(self.ch),
            b')' => tok = Token::RPAREN(self.ch),
            b',' => tok = Token::COMMA(self.ch),
            b'+' => tok = Token::PLUS(self.ch),
            b'{' => tok = Token::LBRACE(self.ch),
            b'}' => tok = Token::RBRACE(self.ch),
            0 => tok = Token::EOF(0),
            _ => {}
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
            Token::ASSIGN(b'='),
            Token::PLUS(b'+'),
            Token::LPAREN(b'('),
            Token::RPAREN(b')'),
            Token::LBRACE(b'{'),
            Token::RBRACE(b'}'),
            Token::COMMA(b','),
            Token::SEMICOLON(b';'),
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            assert!(tok == exp);
        }
    }
}
