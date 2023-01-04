use std::iter::Peekable;
use std::str::Chars;
use crate::{
    token::{Token, TokenType},
};

use crate::token::Span;

pub fn scan(source: &str) -> Result<Vec<Token>, String> {
    let mut scanner = Scanner::new(source);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((c, span)) = scanner.read_char() {
        let token = match c {
            '(' => Ok(Some(Token::new(TokenType::LeftParen, span))),
            ')' => Ok(Some(Token::new(TokenType::RightParen, span))),
            '{' => Ok(Some(Token::new(TokenType::LeftBrace, span))),
            '}' => Ok(Some(Token::new(TokenType::RightBrace, span))),
            ',' => Ok(Some(Token::new(TokenType::Comma, span))),
            '.' => Ok(Some(Token::new(TokenType::Dot, span))),
            '-' => Ok(Some(Token::new(TokenType::Minus, span))),
            '+' => Ok(Some(Token::new(TokenType::Plus, span))),
            ';' => Ok(Some(Token::new(TokenType::Semicolon, span))),
            '*' => Ok(Some(Token::new(TokenType::Star, span))),
            '!' => {
                if scanner.peek_char() == Some('=') {
                    Ok(Some(Token::new(TokenType::BangEqual, span)))
                } else {
                    Ok(Some(Token::new(TokenType::Bang, span)))
                }
            }
            '=' => {
                if scanner.peek_char() == Some('=') {
                    Ok(Some(Token::new(TokenType::EqualEqual, span)))
                } else {
                    Ok(Some(Token::new(TokenType::Equal, span)))
                }
            }
            '<' => {
                if scanner.peek_char() == Some('=') {
                    Ok(Some(Token::new(TokenType::LessEqual, span)))
                } else {
                    Ok(Some(Token::new(TokenType::Less, span)))
                }
            }
            '>' => {
                if scanner.peek_char() == Some('=') {
                    Ok(Some(Token::new(TokenType::GreaterEqual, span)))
                } else {
                    Ok(Some(Token::new(TokenType::Greater, span)))
                }
            }
            '/' => {
                if scanner.peek_char() == Some('/') {
                    while scanner.peek_char() != Some('/') {
                        scanner.read_char();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenType::Slash, span)))
                }
            }
            ' ' | '\r' | '\t' | '\n' => Ok(None), // Ignore
            '"' => {
                match scanner.string() {
                    Ok(string) => {
                        Ok(Some(Token::new(TokenType::String(string), span)))
                    }
                    Err(error) => {
                        Err(error)
                    }
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                match scanner.number(c) {
                    Ok(number) => {
                        Ok(Some(Token::new(TokenType::Number(number), span)))
                    }
                    Err(error) => {
                        Err(error)
                    }
                }
            }
            alpha if is_alpha(alpha) => {
                match scanner.identifier(c) {
                    Ok(identifier) => {
                        if let Some(token_type) = Scanner::match_keyword(&identifier) {
                            Ok(Some(Token::new(token_type, span)))
                        } else {
                            Ok(Some(Token::new(TokenType::Identifier(identifier), span)))
                        }
                    }
                    Err(error) => {
                        Err(error)
                    }
                }
            }
            _ => {
                Err(format!("Unexpected character: {}", c))
            }
        };

        match token {
            Ok(Some(token)) => {
                tokens.push(token);
            }
            Ok(None) => {}
            Err(error) => {
                return Err(error);
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, Span::new(scanner.line)));

    Ok(tokens)
}

struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            source: source.chars().peekable(),
            line: 1,
        }
    }

    pub fn read_char(&mut self) -> Option<(char, Span)> {
        if let Some(c) = self.source.next() {
            let span = Span::new(self.line);
            if c == '\n' {
                self.line += 1;
            }
            Some((c, span))
        } else {
            None
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn string(&mut self) -> Result<String, String> {
        let mut buffer = String::new();

        for c in self.source.by_ref() {
            if c == '\n' {
                self.line += 1;
            }
            if c == '"' {
                return Ok(buffer);
            }
            buffer.push(c);
        }

        Err("Unterminated string".to_owned())
    }

    fn number(&mut self, first_digit: char) -> Result<f64, String> {
        let mut buffer = String::new();
        buffer.push(first_digit);

        while let Some(digit) = self.source.peek() {
            if digit.is_ascii_digit() {
                buffer.push(*digit);
                self.source.next();
            } else {
                break;
            }
        }

        if self.source.peek().copied() == Some('.') {
            buffer.push('.');
            self.source.next();

            while let Some(digit) = self.source.peek() {
                if digit.is_ascii_digit() {
                    buffer.push(*digit);
                    self.source.next();
                } else {
                    break;
                }
            }
        }

        buffer.parse().map_err(|_| format!("Invalid number: {}", buffer))
    }


    fn identifier(&mut self, first_char: char) -> Result<String, String> {
        let mut buffer = String::new();
        buffer.push(first_char);

        while let Some(c) = self.source.peek().copied() {
            if is_alphanumeric(c) {
                buffer.push(c);
                self.source.next();
            } else {
                break;
            }
        }

        Ok(buffer)
    }

    fn match_keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

fn is_alpha(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_digit() || is_alpha(c)
}
