use std::fmt;

use super::ast::{Expr, Stmt};

pub trait ExprVisitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}

pub trait StmtVisitor {
    type Output;

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Self::Output;

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Self::Output;

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then: &Stmt,
        r#else: Option<&Stmt>,
    ) -> Self::Output;

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Output;

    fn visit_var_stmt(&mut self, name: &str, initializer: Option<&Expr>) -> Self::Output;
}

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

impl ExprVisitor<String> for AstPrinter {
    //
    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary { rhs, lhs, op, .. } => self.parenthesize(op, &[lhs, rhs]),
            Expr::Grouping(exp) => self.parenthesize(&"group", &[exp]),
            Expr::Unary(op, expr, ..) => self.parenthesize(op, &[expr]),
            Expr::Literal(lit) => format!("{}", lit),
            Expr::Logical { rhs, lhs, kw, .. } => self.parenthesize(kw, &[lhs, rhs]),
            Expr::Variable(s, ..) => s.to_string(),
            Expr::Assign(name, expr, ..) => self.parenthesize(name, &[expr]),
        }
    }
}
