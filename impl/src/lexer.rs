//! The Vyro lexer: source text -> tokens.

use crate::token::{Tok, Token};

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer { src: src.as_bytes(), pos: 0, line: 1 }
    }

    fn peek(&self) -> u8 {
        if self.pos < self.src.len() { self.src[self.pos] } else { 0 }
    }

    fn peek2(&self) -> u8 {
        if self.pos + 1 < self.src.len() { self.src[self.pos + 1] } else { 0 }
    }

    fn bump(&mut self) -> u8 {
        let c = self.peek();
        self.pos += 1;
        if c == b'\n' {
            self.line += 1;
        }
        c
    }

    fn skip_trivia(&mut self) -> Result<(), String> {
        loop {
            let c = self.peek();
            if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
                self.bump();
            } else if c == b'/' && self.peek2() == b'/' {
                while self.peek() != b'\n' && self.peek() != 0 {
                    self.bump();
                }
            } else if c == b'/' && self.peek2() == b'*' {
                self.bump();
                self.bump();
                loop {
                    if self.peek() == 0 {
                        return Err(format!("line {}: unterminated block comment", self.line));
                    }
                    if self.peek() == b'*' && self.peek2() == b'/' {
                        self.bump();
                        self.bump();
                        break;
                    }
                    self.bump();
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut out = Vec::new();
        loop {
            self.skip_trivia()?;
            let line = self.line;
            let c = self.peek();
            if c == 0 {
                out.push(Token { tok: Tok::Eof, line });
                break;
            }
            let tok = if c.is_ascii_digit() {
                self.number()?
            } else if c == b'"' {
                self.string()?
            } else if c.is_ascii_alphabetic() || c == b'_' {
                self.ident()
            } else {
                self.symbol()?
            };
            out.push(Token { tok, line });
        }
        Ok(out)
    }

    fn number(&mut self) -> Result<Tok, String> {
        let start = self.pos;
        while self.peek().is_ascii_digit() {
            self.bump();
        }
        let mut is_float = false;
        // a single '.' followed by a digit is a fractional part; '..' is a range
        if self.peek() == b'.' && self.peek2() != b'.' && self.peek2().is_ascii_digit() {
            is_float = true;
            self.bump();
            while self.peek().is_ascii_digit() {
                self.bump();
            }
        }
        let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
        if is_float {
            text.parse::<f64>()
                .map(Tok::Float)
                .map_err(|_| format!("line {}: invalid float '{}'", self.line, text))
        } else {
            text.parse::<i64>()
                .map(Tok::Int)
                .map_err(|_| format!("line {}: invalid integer '{}'", self.line, text))
        }
    }

    fn string(&mut self) -> Result<Tok, String> {
        self.bump(); // opening quote
        let mut s = String::new();
        loop {
            let c = self.peek();
            if c == 0 {
                return Err(format!("line {}: unterminated string", self.line));
            }
            if c == b'"' {
                self.bump();
                break;
            }
            if c == b'\\' {
                self.bump();
                let e = self.bump();
                match e {
                    b'n' => s.push('\n'),
                    b't' => s.push('\t'),
                    b'r' => s.push('\r'),
                    b'\\' => s.push('\\'),
                    b'"' => s.push('"'),
                    other => s.push(other as char),
                }
            } else {
                s.push(self.bump() as char);
            }
        }
        Ok(Tok::Str(s))
    }

    fn ident(&mut self) -> Tok {
        let start = self.pos;
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.bump();
        }
        let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
        match text {
            "let" => Tok::Let,
            "const" => Tok::Const,
            "func" => Tok::Func,
            "class" => Tok::Class,
            "if" => Tok::If,
            "else" => Tok::Else,
            "while" => Tok::While,
            "for" => Tok::For,
            "in" => Tok::In,
            "return" => Tok::Return,
            "true" => Tok::True,
            "false" => Tok::False,
            "null" => Tok::Null,
            _ => Tok::Ident(text.to_string()),
        }
    }

    fn symbol(&mut self) -> Result<Tok, String> {
        let line = self.line;
        let c = self.bump();
        let tok = match c {
            b'(' => Tok::LParen,
            b')' => Tok::RParen,
            b'{' => Tok::LBrace,
            b'}' => Tok::RBrace,
            b'[' => Tok::LBracket,
            b']' => Tok::RBracket,
            b',' => Tok::Comma,
            b';' => Tok::Semicolon,
            b'+' => Tok::Plus,
            b'*' => Tok::Star,
            b'/' => Tok::Slash,
            b'%' => Tok::Percent,
            b':' => Tok::Colon,
            b'-' => {
                if self.peek() == b'>' {
                    self.bump();
                    Tok::Arrow
                } else {
                    Tok::Minus
                }
            }
            b'=' => {
                if self.peek() == b'=' {
                    self.bump();
                    Tok::Eq
                } else {
                    Tok::Assign
                }
            }
            b'!' => {
                if self.peek() == b'=' {
                    self.bump();
                    Tok::Ne
                } else {
                    Tok::Bang
                }
            }
            b'<' => {
                if self.peek() == b'=' {
                    self.bump();
                    Tok::Le
                } else {
                    Tok::Lt
                }
            }
            b'>' => {
                if self.peek() == b'=' {
                    self.bump();
                    Tok::Ge
                } else {
                    Tok::Gt
                }
            }
            b'&' => {
                if self.peek() == b'&' {
                    self.bump();
                    Tok::And
                } else {
                    return Err(format!("line {}: unexpected '&' (did you mean '&&'?)", line));
                }
            }
            b'|' => {
                if self.peek() == b'|' {
                    self.bump();
                    Tok::Or
                } else {
                    return Err(format!("line {}: unexpected '|' (did you mean '||'?)", line));
                }
            }
            b'.' => {
                if self.peek() == b'.' {
                    self.bump();
                    Tok::DotDot
                } else {
                    Tok::Dot
                }
            }
            other => {
                return Err(format!("line {}: unexpected character '{}'", line, other as char));
            }
        };
        Ok(tok)
    }
}
