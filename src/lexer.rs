use crate::token::Token;

pub struct Lexer {
    input: Box<[u8]>,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl Lexer {
    pub fn new(input: Box<[u8]>) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
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

    pub fn read_string(&mut self) -> &[u8] {
        let position = self.position + 1;

        loop {
            self.read_char();
            match self.ch {
                Some(b'"') | Some(0) => break,
                _ => (),
            }
        };

        &self.input[position..self.position]
    }

    pub fn skip_whitespace(&mut self) {
        'l: loop {
            match self.ch {
                Some(ch) => {
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
            None => tok = Token::EOF,
            Some(ch) => match ch {
                b'=' => match self.peek_char() {
                    Some(b'=') => {
                        tok = Token::Eq;
                        self.read_char();
                    }
                    _ => tok = Token::Assign,
                },
                b'!' => match self.peek_char() {
                    Some(b'=') => {
                        tok = Token::NotEq;
                        self.read_char();
                    }
                    _ => tok = Token::Bang,
                },
                b':' => match self.peek_char() {
                    Some(b'=') => {
                        tok = Token::Walrus;
                        self.read_char();
                    }
                    _ => tok = Token::Illegal(" ".into()),
                },
                b';' => tok = Token::Semicolon,
                b'(' => tok = Token::LParen,
                b')' => tok = Token::RParen,
                b',' => tok = Token::Comma,
                b'+' => tok = Token::Plus,
                b'-' => tok = Token::Minus,
                b'*' => tok = Token::Asterisk,
                b'/' => tok = Token::Slash,
                b'%' => tok = Token::Percent,
                b'>' => tok = Token::GT,
                b'<' => tok = Token::LT,
                b'{' => tok = Token::LBrace,
                b'}' => tok = Token::RBrace,
                b'"' => {
                    let literal = self.read_string();
                    tok = Token::String(String::from_utf8(literal.to_vec()).unwrap());
                },
                _ => {
                    if self.is_letter() {
                        tok = self.read_identifier();
                        return tok;
                    } else if self.is_digit() {
                        tok = self.read_number();
                        return tok;
                    } else {
                        tok = Token::Illegal(" ".into())
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
            Some(ch) => ch >= b'a' && ch <= b'z' || ch >= b'A' && ch <= b'Z' || ch == b'_',
            None => false,
        }
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.is_digit() {
            self.read_char();
        }

        let num = &self.input[position..self.position];
        let num = std::str::from_utf8(num).unwrap();
        Token::Int(String::from(num))
    }

    pub fn is_digit(&self) -> bool {
        match self.ch {
            Some(ch) => ch >= b'0' && ch <= b'9',
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
        let input = "=+(){},;!-/*5;5 < 10 > 5;:="
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let expected: [Token; 21] = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::GT,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Walrus,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }

    #[test]
    fn multichar_token() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

\"Hello, World!\""
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let expected: [Token; 38] = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::String(String::from("Hello, World!")),
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }

    #[test]
    fn if_else() {
        let input = "if (5 < 10) {
    return true;
} else {
    return false;
}"
        .to_owned()
        .into_bytes()
        .into_boxed_slice();

        let expected: [Token; 18] = [
            Token::If,
            Token::LParen,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }
    #[test]
    fn equality() {
        let input = "10 == 10;\n9 != 10;"
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let expected: [Token; 8] = [
            Token::Int(String::from("10")),
            Token::Eq,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("9")),
            Token::NotEq,
            Token::Int(String::from("10")),
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }
}
