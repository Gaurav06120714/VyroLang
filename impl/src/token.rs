//! Token definitions for the Vyro lexer.

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Ident(String),

    // Keywords
    Let,
    Const,
    Func,
    Class,
    If,
    Else,
    While,
    For,
    In,
    Return,
    True,
    False,
    Null,

    // Punctuation / operators
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Bang,
    And,
    Or,
    DotDot,
    Arrow, // -> (parsed and ignored for return-type annotations)
    Colon, // : (parsed and ignored for type annotations)

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tok: Tok,
    pub line: usize,
}
