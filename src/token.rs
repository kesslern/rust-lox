use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
// TODO: Implement display trait
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Token {
        Token { token_type, span }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl PartialEq<TokenType> for &Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<&TokenType> for Token {
    fn eq(&self, other: &&TokenType) -> bool {
        self.token_type == **other
    }
}

impl Span {
    pub fn new(line: usize) -> Span {
        Span { line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.span();
        write!(f, "{:?} on line {}", self.token_type, span.line)
    }
}
