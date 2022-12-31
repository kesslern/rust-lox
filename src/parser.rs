use crate::{
    ast::Expr,
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(TokenType::Equal) || self.match_token(TokenType::EqualEqual) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), Box::new(op.clone()), Box::new(right));
        }

        expr
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        if self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let op = self.previous();
            let right = self.unary();
            Expr::Unary(Box::new(op), Box::new(right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(TokenType::True) {
            Expr::Literal(Box::new(Literal::Boolean(true)))
        } else if self.match_token(TokenType::False) {
            Expr::Literal(Box::new(Literal::Boolean(false)))
        } else if self.match_token(TokenType::Nil) {
            Expr::Literal(Box::new(Literal::Nil()))
        } else if self.match_token(TokenType::Number) {
            let literal = self.previous().literal.unwrap();
            match literal {
                Literal::Number(n) => Expr::Literal(Box::new(Literal::Number(n))),
                _ => panic!("Expected number literal"), // TODO: Remove panic
            }
        } else if self.match_token(TokenType::String) {
            let literal = self.previous().literal.unwrap();
            match literal {
                Literal::String(s) => Expr::Literal(Box::new(Literal::String(s))),
                _ => panic!("Expected string literal"), // TODO: Remove panic
            }
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            Expr::Grouping(Box::new(expr))
        } else {
            panic!("Expected expression, found {}.", self.peek()); // TODO: Remove panic
        }
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            panic!("{} {}", self.peek(), message) //TODO: Remove panic
        }
    }
}
