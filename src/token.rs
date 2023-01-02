use std::fmt::Display;
use crate::ast::LiteralExpr;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<LiteralExpr>,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            literal: None,
            lexeme,
            line,
        }
    }

    pub fn literal(
        token_type: TokenType,
        literal: LiteralExpr,
        lexeme: String,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            literal: Some(literal),
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} [{}] on line {}",
            self.token_type, self.lexeme, self.line
        )
    }
}
