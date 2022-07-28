use crate::ast::{Expr, Visitor};

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(e1, t, e2) => format!(
                "({} {} {})",
                t.lexeme,
                self.visit_expr(e1),
                self.visit_expr(e2)
            ),
            Expr::Grouping(e) => format!("(group {})", self.visit_expr(e)),
            Expr::Literal(l) => format!("{}", l),
            Expr::Unary(t, e) => format!("({} {})", t.lexeme, self.visit_expr(e)),
        }
    }
}
