use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::{
    ast::Expr,
    token::{Token, TokenType},
};
use crate::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::Expr::{Grouping, Literal};
use crate::parser::ParseError::{ExpectedExpression, ExpectedToken};

pub enum ParseError {
    ExpectedToken(TokenType),
    ExpectedExpression(),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedToken(token_type) => write!(f, "Expected token of type {:?}", token_type),
            ExpectedExpression() => write!(f, "Expected expression"),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut ctx = ParseCtx::new(&tokens);
    expression(&mut ctx)
}

struct ParseCtx<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> ParseCtx<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> ParseCtx<'a> {
        ParseCtx {
            tokens: tokens.iter().peekable(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next().map(|t| t.clone())
    }

    fn peek(&mut self) -> Option<Token> {
        if let Some(token) = self.tokens.peek() {
            let x1: &Token = *token;
            let x2: Token = x1.clone();
            Some(x2)
        } else {
            None
        }
    }

    fn read_token_if(&mut self, token_type: &TokenType) -> Option<Token> {
        let token = self.peek();

        match token {
            Some(token) if token == token_type => {
                self.next();
                Some(token)
            }
            _ => None,
        }
    }

    fn read_token_if_any(&mut self, token_type: &[TokenType]) -> Option<Token> {
        for token in token_type {
            if let Some(token) = self.read_token_if(token) {
                return Some(token);
            }
        }

        None
    }
}

fn expression(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    equality(ctx)
}

fn equality(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    let mut expr = comparison(ctx)?;

    while let Some(op) =  ctx.read_token_if_any(&[TokenType::Equal, TokenType::EqualEqual]) {
        let right = comparison(ctx)?;
        expr = Expr::Binary(BinaryExpr::new(expr, op, right));
    }

    Ok(expr)
}

fn comparison(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    let mut expr = term(ctx)?;

    while let Some(op) = ctx.read_token_if_any(&[
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual])
    {
        let right = term(ctx)?;
        expr = Expr::Binary(BinaryExpr::new(expr, op.clone(), right));
    }

    Ok(expr)
}

fn term(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    let mut expr = factor(ctx)?;

    while let Some(op) = ctx.read_token_if_any(&[TokenType::Minus, TokenType::Plus]) {
        let right = factor(ctx)?;
        expr = Expr::Binary(BinaryExpr::new(expr, op.clone(), right));
    }

    Ok(expr)
}

fn factor(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    let mut expr = unary(ctx)?;

    if let Some(op) = ctx.read_token_if_any(&[TokenType::Star, TokenType::Slash]) {
        let right = factor(ctx)?;
        expr = Expr::Binary(BinaryExpr::new(expr, op.clone(), right));
    }

    Ok(expr)
}

fn unary(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    if let Some(op) = ctx.read_token_if_any(&[TokenType::Bang, TokenType::Minus]) {
        let right = unary(ctx)?;
        Ok(Expr::Unary(UnaryExpr::new(op.clone(), right)))
    } else {
        primary(ctx)
    }
}

fn primary(ctx: &mut ParseCtx) -> Result<Expr, ParseError> {
    let token = ctx.next().expect("Error: Should have a next token");

    match token.token_type() {
        TokenType::True => Ok(Literal(LiteralExpr::Boolean(true))),
        TokenType::False => Ok(Literal(LiteralExpr::Boolean(false))),
        TokenType::Nil => Ok(Literal(LiteralExpr::Nil())),
        TokenType::Number(n) => Ok(Literal(LiteralExpr::Number(*n))),
        TokenType::String(s) => Ok(Literal(LiteralExpr::String(s.to_string()))),
        TokenType::LeftParen => {
            let expr = expression(ctx)?;
            if let Some(_) = ctx.read_token_if(&TokenType::RightParen) {
                Ok(Grouping(GroupingExpr::new(expr)))
            } else {
                Err(ExpectedToken(TokenType::RightParen))
            }
        }
        _ => Err(ExpectedExpression())
    }
}