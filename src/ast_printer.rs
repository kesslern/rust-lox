use crate::ast::{Expr, Visitor};

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(self, expr: &Expr) -> String {
        expr.to_string()
    }
}
