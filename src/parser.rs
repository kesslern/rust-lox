use crate::{
    ast::Expr,
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Clone, Debug)]
pub struct ParseError {
    pub message: String,
    pub token: Option<Token>,
    pub line: Option<usize>,
}

impl ParseError {
    // TODO: Make these constructors into a builder
    fn new(message: String, line: usize) -> ParseError {
        ParseError { message, line: Some(line), token: None }
    }
    
    fn message(message: String) -> ParseError {
        ParseError { message, line: None, token: None }
    }

    fn message_with_token(message: String, token: Token) -> ParseError {
        ParseError { message, line: None, token: Some(token) }
    }
}

// TODO: Revisit https://craftinginterpreters.com/parsing-expressions.html#synchronizing-a-recursive-descent-parser
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(TokenType::Equal) || self.match_token(TokenType::EqualEqual) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op.clone()), Box::new(right));
        }

        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        if self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let op = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(Box::new(op), Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(TokenType::True) {
            Ok(Expr::Literal(Box::new(Literal::Boolean(true))))
        } else if self.match_token(TokenType::False) {
            Ok(Expr::Literal(Box::new(Literal::Boolean(false))))
        } else if self.match_token(TokenType::Nil) {
            Ok(Expr::Literal(Box::new(Literal::Nil())))
        } else if self.match_token(TokenType::Number) {
            let literal = self.previous().literal.unwrap();
            match literal {
                Literal::Number(n) => Ok(Expr::Literal(Box::new(Literal::Number(n)))),
                _ => panic!("Expected number literal"), // TODO: Remove panic
            }
        } else if self.match_token(TokenType::String) {
            let literal = self.previous().literal.unwrap();
            match literal {
                Literal::String(s) => Ok(Expr::Literal(Box::new(Literal::String(s)))),
                _ => Err(ParseError::message("Expected string literal".to_owned()))
            }
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(ParseError::message(format!("Expected expression, found {}.", self.peek())))
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::message(format!("{} {}", self.peek(), message)))
        }
    }
}
