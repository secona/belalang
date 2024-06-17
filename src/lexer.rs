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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let tok: Token = match self.ch {
            None => Token::EOF,
            Some(ch) => match ch {
                b'=' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Eq
                    }
                    _ => Token::Assign,
                },
                b'!' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Ne
                    }
                    _ => Token::Not,
                },
                b':' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::ColonAssign
                    }
                    _ => Token::Illegal(" ".into()),
                },
                b'<' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Le
                    },
                    _ => Token::Lt,
                },
                b'>' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Ge
                    },
                    _ => Token::Gt,
                },
                b';' => Token::Semicolon,
                b'(' => Token::LeftParen,
                b')' => Token::RightParen,
                b',' => Token::Comma,
                b'+' => Token::Add,
                b'-' => Token::Sub,
                b'*' => Token::Mul,
                b'/' => Token::Div,
                b'%' => Token::Mod,
                b'{' => Token::LeftBrace,
                b'}' => Token::RightBrace,
                b'"' => self.read_string(),
                _ => {
                    if self.is_letter() {
                        return self.read_identifier();
                    } else if self.is_digit() {
                        return self.read_number();
                    } else {
                        Token::Illegal(" ".into())
                    }
                }
            },
        };

        self.read_char();
        tok
    }

    pub fn read_char(&mut self) -> Option<&'a u8> {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(&self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.ch
    }

    pub fn peek_char(&self) -> Option<&'a u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(&self.input[self.read_position])
        }
    }

    pub fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.ch {
                Some(b'#') => self.skip_comment(),
                Some(b' ' | b'\t' | b'\n' | b'\r') => {
                    self.read_char();
                }
                _ => break,
            };
        }
    }

    pub fn skip_comment(&mut self) {
        while let Some(b'#') = self.ch {
            loop {
                match self.ch {
                    Some(b'\n') | None => {
                        self.read_char();
                        break;
                    }
                    _ => self.read_char(),
                };
            }
        }
    }

    pub fn read_string(&mut self) -> Token {
        let position = self.position + 1;

        loop {
            self.read_char();
            match self.ch {
                Some(b'"') | Some(0) => break,
                _ => (),
            }
        }

        let s = &self.input[position..self.position];
        let s = std::str::from_utf8(s).unwrap();
        Token::String(String::from(s))
    }

    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while self.is_letter() {
            self.read_char();
        }

        Token::from(&self.input[position..self.position])
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

    pub fn is_letter(&self) -> bool {
        if let Some(ch) = self.ch {
            *ch >= b'a' && *ch <= b'z' || *ch >= b'A' && *ch <= b'Z' || *ch == b'_'
        } else {
            false
        }
    }

    pub fn is_digit(&self) -> bool {
        match self.ch {
            Some(ch) => *ch >= b'0' && *ch <= b'9',
            None => false,
        }
    }
}
