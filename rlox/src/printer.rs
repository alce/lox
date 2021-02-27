use std::fmt;

use crate::ast::{BinOp, Expr, Keyword, Lit, UnOp};
use crate::visitor::ExprVisitor;

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

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary { rhs, lhs, op, line } => self.visit_binary_expr(lhs, *op, rhs, *line),
            Expr::Call { .. } => todo!(),
            Expr::Grouping(expr) => self.visit_grouping_expr(expr),
            Expr::Unary(op, expr, line) => self.visit_unary_expr(*op, expr, *line),
            Expr::Literal(lit) => self.visit_literal_expr(lit),
            Expr::Logical { rhs, lhs, kw, line } => self.visit_logical_expr(lhs, *kw, rhs, *line),
            Expr::Variable(s, line) => self.visit_variable_expr(s, *line),
            Expr::Assign(name, expr, line) => self.visit_assign_expr(name, expr, *line),
        }
    }

    fn visit_assign_expr(&mut self, name: &str, expr: &Expr, _: u64) -> Self::Output {
        self.parenthesize(&name, &[expr])
    }

    fn visit_binary_expr(&mut self, lhs: &Expr, op: BinOp, rhs: &Expr, _: u64) -> Self::Output {
        self.parenthesize(&op, &[lhs, rhs])
    }

    fn visit_call_expr(&mut self, _callee: &Expr, _args: &[Expr], _: u64) -> Self::Output {
        todo!()
    }

    fn visit_literal_expr(&mut self, literal: &Lit) -> Self::Output {
        format!("{}", literal)
    }

    fn visit_logical_expr(&mut self, lhs: &Expr, kw: Keyword, rhs: &Expr, _: u64) -> Self::Output {
        self.parenthesize(&kw, &[lhs, rhs])
    }

    fn visit_unary_expr(&mut self, op: UnOp, rhs: &Expr, _: u64) -> Self::Output {
        self.parenthesize(&op, &[rhs])
    }

    fn visit_variable_expr(&mut self, name: &str, _: u64) -> Self::Output {
        name.into()
    }
}
