use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<&'a u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) -> Option<&u8> {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(&self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.ch
    }

    pub fn peek_char(&self) -> Option<&u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(&self.input[self.read_position])
        }
    }

    pub fn skip_whitespace(&mut self) {
        'l: loop {
            match self.ch {
                Some(ch) => {
                    let ch = *ch;
                    if ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r' {
                        self.read_char();
                    } else {
                        break 'l;
                    }
                }
                None => break 'l,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token;

        match self.ch {
            None => tok = Token::EOF(),
            Some(ch) => match ch {
                b'=' => match self.peek_char() {
                    Some(pk) if *pk == b'=' => {
                        let ch = &self.input[self.position..=self.read_position];
                        tok = Token::EQ(ch);
                        self.read_char();
                    }
                    _ => tok = Token::ASSIGN(ch),
                },
                b';' => tok = Token::SEMICOLON(ch),
                b'(' => tok = Token::LPAREN(ch),
                b')' => tok = Token::RPAREN(ch),
                b',' => tok = Token::COMMA(ch),
                b'+' => tok = Token::PLUS(ch),
                b'-' => tok = Token::MINUS(ch),
                b'!' => match self.peek_char() {
                    Some(pk) if *pk == b'=' => {
                        let ch = &self.input[self.position..=self.read_position];
                        tok = Token::NOTEQ(ch);
                        self.read_char();
                    }
                    _ => tok = Token::BANG(ch),
                },
                b'*' => tok = Token::ASTERISK(ch),
                b'/' => tok = Token::SLASH(ch),
                b'>' => tok = Token::GT(ch),
                b'<' => tok = Token::LT(ch),
                b'{' => tok = Token::LBRACE(ch),
                b'}' => tok = Token::RBRACE(ch),
                _ => {
                    if self.is_letter() {
                        tok = self.read_identifier();
                        return tok;
                    } else if self.is_digit() {
                        tok = self.read_number();
                        return tok;
                    } else {
                        tok = Token::ILLEGAL(&b' ')
                    }
                }
            },
        };

        self.read_char();
        tok
    }

    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while self.is_letter() {
            self.read_char();
        }

        Token::lookup_ident(&self.input[position..self.position])
    }

    pub fn is_letter(&self) -> bool {
        match self.ch {
            Some(ch) => {
                let ch = *ch;
                ch >= b'a' && ch <= b'z' || ch >= b'A' && ch <= b'Z' || ch == b'_'
            }
            None => false,
        }
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.is_digit() {
            self.read_char();
        }

        Token::INT(&self.input[position..self.position])
    }

    pub fn is_digit(&self) -> bool {
        match self.ch {
            Some(ch) => {
                let ch = *ch;
                ch >= b'0' && ch <= b'9'
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn tokens() {
        let input = b"=+(){},;!-/*5;5 < 10 > 5;";

        let expected: [Token; 20] = [
            Token::ASSIGN(&b'='),
            Token::PLUS(&b'+'),
            Token::LPAREN(&b'('),
            Token::RPAREN(&b')'),
            Token::LBRACE(&b'{'),
            Token::RBRACE(&b'}'),
            Token::COMMA(&b','),
            Token::SEMICOLON(&b';'),
            Token::BANG(&b'!'),
            Token::MINUS(&b'-'),
            Token::SLASH(&b'/'),
            Token::ASTERISK(&b'*'),
            Token::INT(b"5"),
            Token::SEMICOLON(&b';'),
            Token::INT(b"5"),
            Token::LT(&b'<'),
            Token::INT(b"10"),
            Token::GT(&b'>'),
            Token::INT(b"5"),
            Token::SEMICOLON(&b';'),
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert!(tok == exp);
        }
    }

    #[test]
    fn multichar_token() {
        let input = b"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";

        let expected: [Token; 37] = [
            Token::LET(b"let"),
            Token::IDENT(b"five"),
            Token::ASSIGN(&b'='),
            Token::INT(b"5"),
            Token::SEMICOLON(&b';'),
            Token::LET(b"let"),
            Token::IDENT(b"ten"),
            Token::ASSIGN(&b'='),
            Token::INT(b"10"),
            Token::SEMICOLON(&b';'),
            Token::LET(b"let"),
            Token::IDENT(b"add"),
            Token::ASSIGN(&b'='),
            Token::FUNCTION(b"fn"),
            Token::LPAREN(&b'('),
            Token::IDENT(b"x"),
            Token::COMMA(&b','),
            Token::IDENT(b"y"),
            Token::RPAREN(&b')'),
            Token::LBRACE(&b'{'),
            Token::IDENT(b"x"),
            Token::PLUS(&b'+'),
            Token::IDENT(b"y"),
            Token::SEMICOLON(&b';'),
            Token::RBRACE(&b'}'),
            Token::SEMICOLON(&b';'),
            Token::LET(b"let"),
            Token::IDENT(b"result"),
            Token::ASSIGN(&b'='),
            Token::IDENT(b"add"),
            Token::LPAREN(&b'('),
            Token::IDENT(b"five"),
            Token::COMMA(&b','),
            Token::IDENT(b"ten"),
            Token::RPAREN(&b')'),
            Token::SEMICOLON(&b';'),
            Token::EOF(),
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert!(tok == exp);
        }
    }

    #[test]
    fn if_else() {
        let input = b"if (5 < 10) {
    return true;
} else {
    return false;
}";

        let expected: [Token; 18] = [
            Token::IF(b"if"),
            Token::LPAREN(&b'('),
            Token::INT(b"5"),
            Token::LT(&b'<'),
            Token::INT(b"10"),
            Token::RPAREN(&b')'),
            Token::LBRACE(&b'{'),
            Token::RETURN(b"return"),
            Token::TRUE(b"true"),
            Token::SEMICOLON(&b';'),
            Token::RBRACE(&b'}'),
            Token::ELSE(b"else"),
            Token::LBRACE(&b'{'),
            Token::RETURN(b"return"),
            Token::FALSE(b"false"),
            Token::SEMICOLON(&b';'),
            Token::RBRACE(&b'}'),
            Token::EOF(),
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert!(tok == exp);
        }
    }
    #[test]
    fn equality() {
        let input = b"10 == 10;
9 != 10;";

        let expected: [Token; 8] = [
            Token::INT(b"10"),
            Token::EQ(b"=="),
            Token::INT(b"10"),
            Token::SEMICOLON(&b';'),
            Token::INT(b"9"),
            Token::NOTEQ(b"!="),
            Token::INT(b"10"),
            Token::SEMICOLON(&b';'),
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert!(tok == exp);
        }
    }
}
