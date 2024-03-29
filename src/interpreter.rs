use crate::ast::Expr::Literal;
use crate::ast::{Expr, LiteralExpr};
use crate::interpreter::RuntimeError::{
    ExpectedNumberLiterals, InvalidLiteralForUnary, InvalidOperandError, UnknownOperatorError,
};
use crate::token::{Token, TokenType};
use std::fmt::{Display, Formatter};

pub enum RuntimeError {
    ExpectedNumberLiterals(String),
    UnknownOperatorError(Token),
    InvalidOperandError(Token),
    InvalidLiteralForUnary(Token),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedNumberLiterals(msg) => write!(f, "{}", msg),
            UnknownOperatorError(token) => write!(f, "Unknown operator: {}", token),
            InvalidOperandError(token) => write!(f, "Invalid operand: {}", token),
            InvalidLiteralForUnary(token) => write!(f, "Invalid literal for unary: {}", token),
        }
    }
}

fn is_truthy(literal: &LiteralExpr) -> bool {
    match literal {
        LiteralExpr::Nil() => false,
        LiteralExpr::Boolean(b) => *b,
        _ => true,
    }
}

fn is_equal(l1: &LiteralExpr, l2: &LiteralExpr) -> bool {
    match (l1, l2) {
        (LiteralExpr::Nil(), LiteralExpr::Nil()) => true,
        (LiteralExpr::Boolean(b1), LiteralExpr::Boolean(b2)) => b1 == b2,
        (LiteralExpr::Number(n1), LiteralExpr::Number(n2)) => (n1 - n2).abs() < 0.01,
        (LiteralExpr::String(s1), LiteralExpr::String(s2)) => s1 == s2,
        _ => false,
    }
}

fn check_number_operands(
    operator: &str,
    left: &LiteralExpr,
    right: &LiteralExpr,
    func: fn(f64, f64) -> LiteralExpr,
) -> Result<LiteralExpr, RuntimeError> {
    if let (LiteralExpr::Number(n1), LiteralExpr::Number(n2)) = (left, right) {
        Ok(func(*n1, *n2))
    } else {
        Err(ExpectedNumberLiterals(operator.to_string()))
    }
}

pub fn interpret(expression: &Expr) -> Result<(), RuntimeError> {
    match interpret_expr(expression) {
        Ok(literal_expr) => {
            println!("{}", literal_expr);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn interpret_expr(expr: &Expr) -> Result<LiteralExpr, RuntimeError> {
    match expr {
        Expr::Binary(expr) => {
            let left = interpret_expr(expr.left())?;
            let right = interpret_expr(expr.right())?;

            match expr.op().token_type() {
                TokenType::Plus => match (left, right) {
                    (LiteralExpr::Number(n1), LiteralExpr::Number(n2)) => {
                        Ok(LiteralExpr::Number(n1 + n2))
                    }
                    (LiteralExpr::String(s1), LiteralExpr::String(s2)) => {
                        Ok(LiteralExpr::String(format!("{}{}", s1, s2)))
                    }
                    _ => Err(InvalidOperandError(expr.op().clone())),
                },
                TokenType::Minus => {
                    check_number_operands("-", &left, &right, |n1, n2| LiteralExpr::Number(n1 - n2))
                }
                TokenType::Star => {
                    check_number_operands("*", &left, &right, |n1, n2| LiteralExpr::Number(n1 * n2))
                }
                TokenType::Slash => {
                    check_number_operands("/", &left, &right, |n1, n2| LiteralExpr::Number(n1 / n2))
                }
                TokenType::Greater => check_number_operands(">", &left, &right, |n1, n2| {
                    LiteralExpr::Boolean(n1 > n2)
                }),
                TokenType::GreaterEqual => check_number_operands(">=", &left, &right, |n1, n2| {
                    LiteralExpr::Boolean(n1 >= n2)
                }),
                TokenType::Less => check_number_operands("<", &left, &right, |n1, n2| {
                    LiteralExpr::Boolean(n1 < n2)
                }),
                TokenType::LessEqual => check_number_operands("<=", &left, &right, |n1, n2| {
                    LiteralExpr::Boolean(n1 <= n2)
                }),
                TokenType::EqualEqual => Ok(LiteralExpr::Boolean(is_equal(&left, &right))),
                TokenType::BangEqual => Ok(LiteralExpr::Boolean(!is_equal(&left, &right))),
                _ => Err(UnknownOperatorError(expr.op().clone())),
            }
        }
        Expr::Grouping(e) => interpret_expr(e.expression()),
        Literal(l) => Ok(l.clone()),
        Expr::Unary(u) => {
            let right = interpret_expr(u.expr())?;
            match u.op().token_type() {
                TokenType::Minus => {
                    if let LiteralExpr::Number(n) = right {
                        Ok(LiteralExpr::Number(-n))
                    } else {
                        Err(InvalidLiteralForUnary(u.op().clone()))
                    }
                }
                TokenType::Bang => Ok(LiteralExpr::Boolean(!is_truthy(&right))),
                _ => Err(UnknownOperatorError(u.op().clone())),
            }
        }
    }
}
