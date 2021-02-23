use std::fmt;

use super::ast::Expr;

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}

// This is dodgy
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        self.visit_expr(expr)
    }

    fn parenthesize(&mut self, name: &impl fmt::Display, expr: &[&Expr]) -> String {
        let ex = expr.iter().fold(String::new(), |mut s, e| {
            s.push(' ');
            s.push_str(&self.visit_expr(e));
            s
        });

        format!("({}{})", name, ex)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary { rhs, lhs, op } => self.parenthesize(op, &[lhs, rhs]),
            Expr::Grouping(exp) => self.parenthesize(&"group", &[exp]),
            Expr::Unary(op, expr) => self.parenthesize(op, &[expr]),
            Expr::Literal(lit) => format!("{}", lit),
        }
    }
}
