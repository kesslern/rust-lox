use crate::{
    ast::Expr,
    token::{Token, TokenType},
};
use crate::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::Expr::{Grouping, Literal};
use crate::error::{Error, ErrorBuilder, ErrorType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// TODO: Revisit https://craftinginterpreters.com/parsing-expressions.html#synchronizing-a-recursive-descent-parser
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.match_token(TokenType::Equal) || self.match_token(TokenType::EqualEqual) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == &token_type
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        if self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let op = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr::new(op, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        let token = self.advance();

        match token.token_type() {
            TokenType::True => Ok(Literal(LiteralExpr::Boolean(true))),
            TokenType::False => Ok(Literal(LiteralExpr::Boolean(false))),
            TokenType::Nil => Ok(Literal(LiteralExpr::Nil())),
            TokenType::Number(n) => Ok(Literal(LiteralExpr::Number(*n))),
            TokenType::String(s) => Ok(Literal(LiteralExpr::String(s.to_string()))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Grouping(GroupingExpr::new(expr)))
            }
            _ => Err(Parser::error_builder("Expected expression")
                .token(self.peek())
                .build())
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
        self.peek().token_type() == &TokenType::Eof
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

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(Parser::error_builder(error_message)
                .token(self.peek())
                .build())
        }
    }

    fn error_builder(message: &str) -> ErrorBuilder {
        ErrorBuilder::new(ErrorType::Parse, message.to_owned())
    }
}
