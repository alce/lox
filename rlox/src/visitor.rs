use super::ast::{Expr, Stmt};
use crate::ast::{BinOp, Keyword, Lit, UnOp};

pub trait ExprVisitor {
    type Output;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
        match expr {
            Expr::Assign(name, expr, line) => self.visit_assign_expr(name, expr, *line),
            Expr::Binary { lhs, op, rhs, line } => self.visit_binary_expr(lhs, *op, rhs, *line),
            Expr::Call { callee, args, line } => self.visit_call_expr(callee, &args, *line),
            Expr::Grouping(expr) => self.visit_grouping_expr(expr),
            Expr::Literal(lit) => self.visit_literal_expr(lit),
            Expr::Logical { lhs, kw, rhs, line } => self.visit_logical_expr(lhs, *kw, rhs, *line),
            Expr::Unary(op, expr, line) => self.visit_unary_expr(*op, expr, *line),
            Expr::Variable(name, line) => self.visit_variable_expr(name, *line),
        }
    }

    fn visit_assign_expr(&mut self, name: &str, expr: &Expr, line: u64) -> Self::Output;

    fn visit_binary_expr(&mut self, lhs: &Expr, op: BinOp, rhs: &Expr, line: u64) -> Self::Output;

    fn visit_call_expr(&mut self, callee: &Expr, args: &[Expr], line: u64) -> Self::Output;

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Self::Output {
        self.visit_expr(expr)
    }

    fn visit_literal_expr(&mut self, literal: &Lit) -> Self::Output;

    fn visit_logical_expr(
        &mut self,
        lhs: &Expr,
        kw: Keyword,
        rhs: &Expr,
        line: u64,
    ) -> Self::Output;

    fn visit_unary_expr(&mut self, op: UnOp, rhs: &Expr, line: u64) -> Self::Output;

    fn visit_variable_expr(&mut self, name: &str, line: u64) -> Self::Output;
}

pub trait StmtVisitor {
    type Output;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
        match stmt {
            Stmt::Block(stmts) => self.visit_block_stmt(&stmts),
            Stmt::Expr(expr) => self.visit_expression_stmt(expr),
            Stmt::Function {
                name,
                params,
                body,
                line,
            } => self.visit_function_stmt(name, params.as_slice(), body, *line),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Return(expr, line) => self.visit_return_stmt(expr.as_ref(), *line),
            Stmt::Var(name, initializer) => self.visit_var_stmt(name, initializer.as_ref()),
            Stmt::If {
                condition,
                then,
                r#else,
            } => self.visit_if_stmt(condition, then, r#else.as_deref()),
            Stmt::While(condition, body) => self.visit_while_stmt(condition, body),
        }
    }

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Self::Output;

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Self::Output;

    fn visit_function_stmt(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        line: u64,
    ) -> Self::Output;

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then: &Stmt,
        r#else: Option<&Stmt>,
    ) -> Self::Output;

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Output;

    fn visit_return_stmt(&mut self, expr: Option<&Expr>, line: u64) -> Self::Output;

    fn visit_var_stmt(&mut self, name: &str, initializer: Option<&Expr>) -> Self::Output;

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Self::Output;
}
