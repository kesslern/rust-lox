use std::fmt::Display;

use crate::token::Token;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(b) => b.fmt(f),
            Expr::Grouping(g) => g.fmt(f),
            Expr::Literal(l) => l.fmt(f),
            Expr::Unary(u) => u.fmt(f),
        }
    }
}

pub struct BinaryExpr {
    left: Box<Expr>,
    op: Box<Token>,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op: Box::new(op),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expr {
        &self.left
    }

    pub fn op(&self) -> &Token {
        &self.op
    }

    pub fn right(&self) -> &Expr {
        &self.right
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }

    pub fn expression(&self) -> &Expr {
        &self.expression
    }
}

impl Display for GroupingExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

#[derive(Clone, Debug)]
pub enum LiteralExpr {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil(),
}

impl Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralExpr::String(s) => s.fmt(f),
            LiteralExpr::Number(n) => n.fmt(f),
            LiteralExpr::Boolean(b) => b.fmt(f),
            LiteralExpr::Nil() => write!(f, "nil"),
        }
    }
}

pub struct UnaryExpr {
    op: Box<Token>,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: Token, expr: Expr) -> Self {
        Self {
            op: Box::new(op),
            expr: Box::new(expr),
        }
    }

    pub fn op(&self) -> &Token {
        &self.op
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.expr)
    }
}
