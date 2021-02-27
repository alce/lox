use crate::ast::{BinOp, Expr, Keyword, Lit, Stmt, UnOp};
use crate::visitor::{ExprVisitor, StmtVisitor};
use crate::{Interpreter, LoxError};
use std::collections::HashMap;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    #[allow(unused)]
    pub fn new(interpreter: &mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(), LoxError> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        self.visit_stmt(stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LoxError> {
        self.visit_expr(expr)
    }

    fn resolve_function(&mut self, params: &[String], body: &[Stmt]) -> Result<(), LoxError> {
        self.begin_scope();

        for param in params {
            self.declare(param);
            self.define(param);
        }

        self.resolve(body)?;
        self.end_scope();

        Ok(())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_local(&mut self, name: &str) -> Result<(), LoxError> {
        for (i, scope) in self.scopes.iter().enumerate() {
            if scope.contains_key(name) {
                self.interpreter.resolve(name, self.scopes.len() - 1 - i);
                break;
            }
        }

        Ok(())
    }

    fn declare(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.into(), false);
        }
    }

    fn define(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.into(), true);
        }
    }
}

impl StmtVisitor for Resolver<'_> {
    type Output = Result<(), LoxError>;

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Self::Output {
        self.begin_scope();
        self.resolve(stmts)?;
        self.end_scope();

        Ok(())
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Self::Output {
        self.resolve_expr(expr)
    }

    fn visit_function_stmt(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        _line: u64,
    ) -> Self::Output {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then: &Stmt,
        r#else: Option<&Stmt>,
    ) -> Self::Output {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then)?;

        if let Some(stmt) = r#else {
            self.resolve_stmt(stmt)?
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Output {
        self.resolve_expr(expr)
    }

    fn visit_return_stmt(&mut self, expr: Option<&Expr>, _: u64) -> Self::Output {
        if let Some(exp) = expr {
            self.resolve_expr(exp)?
        }

        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &str, initializer: Option<&Expr>) -> Self::Output {
        self.declare(name);
        if let Some(expr) = initializer {
            self.resolve_expr(expr)?;
        }
        self.define(name);

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)
    }
}

impl ExprVisitor for Resolver<'_> {
    type Output = Result<(), LoxError>;

    fn visit_assign_expr(&mut self, name: &str, expr: &Expr, _: u64) -> Self::Output {
        self.resolve_expr(expr)?;
        self.resolve_local(name)
    }

    fn visit_binary_expr(&mut self, lhs: &Expr, _: BinOp, rhs: &Expr, _: u64) -> Self::Output {
        self.resolve_expr(lhs)?;
        self.resolve_expr(rhs)
    }

    fn visit_call_expr(&mut self, callee: &Expr, args: &[Expr], _: u64) -> Self::Output {
        self.resolve_expr(callee)?;
        for arg in args {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_literal_expr(&mut self, _: &Lit) -> Self::Output {
        Ok(())
    }

    fn visit_logical_expr(&mut self, lhs: &Expr, _: Keyword, rhs: &Expr, _: u64) -> Self::Output {
        self.resolve_expr(lhs)?;
        self.resolve_expr(rhs)
    }

    fn visit_unary_expr(&mut self, _: UnOp, rhs: &Expr, _: u64) -> Self::Output {
        self.resolve_expr(rhs)
    }

    fn visit_variable_expr(&mut self, name: &str, _: u64) -> Self::Output {
        if let Some(false) = self.scopes.last().and_then(|scope| scope.get(name)) {
            let msg = "Can't read variable in its own initializer.";
            return Err(LoxError::Compile(msg.into()));
        }

        self.resolve_local(name)
    }
}
