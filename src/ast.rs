use std::fmt::Display;

use crate::token::{Literal, Token};

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Box<Literal>),
    Unary(Box<Token>, Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_expr(&self, e: &Expr) -> T;
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(e1, t, e2) => write!(f, "({} {} {})", t, e1, e2),
            Expr::Grouping(e) => write!(f, "(group {})", e),
            Expr::Literal(l) => write!(f, "{}", l),
            Expr::Unary(t, e) => write!(f, "({} {})", t, e),
        }
    }
}
