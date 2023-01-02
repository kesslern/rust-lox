use crate::{
    lox::Lox,
    token::{Token, TokenType},
};

use crate::ast::LiteralExpr;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token);
            }
            '=' => {
                let token = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token);
            }
            '<' => {
                let token = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token);
            }
            '>' => {
                let token = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token);
            }
            ' ' | '\r' | '\t' => (), // Ignore
            '\n' => self.line += 1,
            '"' => self.string(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),
            _ => {
                if Scanner::is_alpha(Some(c)) {
                    self.identifier();
                } else {
                    Lox::report(Some(self.line), None, &format!("Unexpected character: {}", c))
                    // TODO: how does this break out of the loop?
                }
            }
        };
    }

    fn advance(&mut self) -> char {
        let result = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        result
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source.as_bytes()[self.current] as char)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source.as_bytes()[self.current + 1] as char)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(token_type, text.to_string(), self.line));
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: LiteralExpr) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token::literal(
            token_type,
            literal,
            text.to_string(),
            self.line,
        ));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            if self.is_at_end() {
                // TODO: How does this break out of the loop?
                Lox::report(Some(self.line), None, "Unterminated string.");
                return;
            }

            self.advance();
        }

        // Consume the closing quote
        self.advance();

        let literal = LiteralExpr::String(self.source[self.start + 1..self.current - 1].to_string());

        self.add_token_literal(TokenType::String, literal);
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let literal = LiteralExpr::Number(
            self.source[self.start..self.current]
                .parse::<f64>()
                .unwrap(),
        );

        self.add_token_literal(TokenType::Number, literal);
    }

    fn is_digit(c: Option<char>) -> bool {
        matches!(c, Some('0'..='9'))
    }

    fn is_alpha(c: Option<char>) -> bool {
        matches!(c, Some('a'..='z') | Some('A'..='Z') | Some('_'))
    }

    fn is_alphanumeric(c: Option<char>) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type = match Scanner::match_keyword(text) {
            Some(token_type) => token_type,
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
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
