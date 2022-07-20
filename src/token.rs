use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
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
    Identifier,
    String,
    Number,

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

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal<'a>>,
    line: usize,
}

impl Token<'_> {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        return Token {
            token_type,
            lexeme,
            literal,
            line,
        };
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self.literal {
            Some(l) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, l),
            None => write!(f, "{:?} {}", self.token_type, self.lexeme),
        };
    }
}

pub enum LiteralType {
    String,
    Number,
}

// TODO: Remove these pubs
pub struct Literal<'a> {
    pub literal_type: LiteralType,
    pub value: LiteralValue<'a>,
}

// TODO: Remove these pubs
pub union LiteralValue<'a> {
    pub number: f64,
    pub string: &'a str,
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.literal_type {
                LiteralType::String => write!(f, "\"{}\"", self.value.string),
                LiteralType::Number => write!(f, "{}", self.value.number),
            }
        }
    }
}
