// use crate::ast::{Expr, Visitor};
// use crate::ast::Expr::Literal;
// use crate::token::Literal;
//
// pub struct Interpreter;
//
// impl Interpreter {
//     fn is_truthy(literal: &Literal) -> bool {
//         match literal {
//             Literal::Nil() => false,
//             Literal::Boolean(b) => *b,
//             _ => true,
//         }
//     }
//
//     fn is_equal(l1: &Literal, l2: &Literal) -> bool {
//         match (l1, l2) {
//             (Literal::Nil(), Literal::Nil()) => true,
//             (Literal::Boolean(b1), Literal::Boolean(b2)) => b1 == b2,
//             (Literal::Number(n1), Literal::Number(n2)) => n1 == n2,
//             (Literal::String(s1), Literal::String(s2)) => s1 == s2,
//             _ => false,
//         }
//     }
//
//     fn check_number_operands(operator: &str, left: &Literal, right: &Literal, func: fn(f64, f64) -> Literal) -> Result<Literal, String> {
//         if let (Literal::Number(n1), Literal::Number(n2)) = (left, right) {
//             Ok(func(*n1, *n2))
//         } else {
//             Err(format!("Expected number literals for operand {}", operator))
//         }
//     }
//
//     pub fn interpret(&self, expression: Expr) -> Result<(), String> {
//         match self.visit_expr(&expression) {
//             Ok(literal) => {
//                 println!("{}", literal);
//                 Ok(())
//             }
//             Err(message) => Err(message),
//         }
//     }
// }
//
// impl Visitor<Result<Literal, String>> for Interpreter {
//     fn visit_expr(&self, expr: &Expr) -> Result<Literal, String> {
//         match expr {
//             Expr::Binary(expr) => {
//                 let left = self.visit_expr(expr.left())?;
//                 let right = self.visit_expr(expr.right())?;
//
//                 // TODO: Make these enum values instead of raw strings
//                 match expr.op().lexeme.as_str() {
//                     "+" => {
//                         match (left, right) {
//                             (Literal::Number(n1), Literal::Number(n2)) =>
//                                 Ok(Literal::Number(n1 + n2)),
//                             (Literal::String(s1), Literal::String(s2)) => {
//                                 Ok(Literal::String(format!("{}{}", s1, s2)))
//                             }
//                             _ => Err("Invalid operands for +".to_string()),
//                         }
//                     }
//                     "-" => {
//                         Interpreter::check_number_operands(
//                             "-", &left, &right,
//                             |n1, n2| Literal::Number(n1 - n2))
//                     }
//                     "*" => {
//                         Interpreter::check_number_operands(
//                             "*", &left, &right,
//                             |n1, n2| Literal::Number(n1 * n2))
//                     }
//                     "/" => {
//                         Interpreter::check_number_operands(
//                             "/", &left, &right,
//                             |n1, n2| Literal::Number(n1 / n2))
//                     }
//                     ">" => {
//                         Interpreter::check_number_operands(
//                             ">", &left, &right,
//                             |n1, n2| Literal::Boolean(n1 > n2))
//                     }
//                     ">=" => {
//                         Interpreter::check_number_operands(
//                             ">=", &left, &right,
//                             |n1, n2| Literal::Boolean(n1 >= n2))
//                     }
//                     "<" => {
//                         Interpreter::check_number_operands(
//                             "<", &left, &right,
//                             |n1, n2| Literal::Boolean(n1 < n2))
//                     }
//                     "<=" => {
//                         Interpreter::check_number_operands(
//                             "<=", &left, &right,
//                             |n1, n2| Literal::Boolean(n1 <= n2))
//                     }
//                     "==" => Ok(Literal::Boolean(Interpreter::is_equal(&left, &right))),
//                     "!=" => Ok(Literal::Boolean(!Interpreter::is_equal(&left, &right))),
//                     _ => Err(format!("Unknown operator {}", t.lexeme)),
//                 }
//             }
//             Expr::Grouping(e) => {
//                 self.visit_expr(e)
//             }
//             Expr::Literal(l) => {
//                 Ok(*l.clone())
//             }
//             Expr::Unary(t, e) => {
//                 let right = self.visit_expr(e)?;
//                 match t.lexeme.as_str() {
//                     "-" => {
//                         if let Literal::Number(n) = right {
//                             Ok(Literal::Number(-n))
//                         } else {
//                             Err(format!("Expected number literal for operand {}", t.lexeme))
//                         }
//                     }
//                     "!" => Ok(Literal::Boolean(!Interpreter::is_truthy(&right))),
//                     _ => Err(format!("Unknown operator {}", t.lexeme)),
//                 }
//             }
//         }
//     }
// }