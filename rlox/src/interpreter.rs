use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::ast::{BinOp, Expr, Keyword, Stmt, UnOp};
use crate::clock::Clock;
use crate::env::Env;
use crate::function::Func;
use crate::value::Value;
use crate::visitor::{ExprVisitor, StmtVisitor};
use crate::LoxError;

pub struct Interpreter {
    env: Rc<RefCell<Env>>,
    globals: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = Env::new();
        env.define("clock", Value::Call(Rc::new(Clock)));
        let globals = Rc::new(RefCell::new(env));

        Interpreter {
            env: globals.clone(),
            globals,
        }
    }

    pub fn globals(&self) -> Rc<RefCell<Env>> {
        self.globals.clone()
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in stmts {
            self.execute(&stmt)?
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError> {
        self.visit_expr(expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
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

    pub fn execute_block(&mut self, stmts: &[Stmt], env: Rc<RefCell<Env>>) -> Result<(), LoxError> {
        let prev = mem::replace(&mut self.env, env);
        for stmt in stmts {
            if let Err(e) = self.execute(stmt) {
                self.env = prev;
                return Err(e);
            }
        }

        self.env = prev;
        Ok(())
    }

    fn assign_expr(&mut self, name: &str, expr: &Expr, line: u64) -> Result<Value, LoxError> {
        let val = self.evaluate(expr)?;
        self.env
            .borrow_mut()
            .assign(name, val.clone())
            .map_err(|e| LoxError::Runtime(e, line))?;

        Ok(val)
    }

    fn binary_expr(
        &mut self,
        lhs: &Expr,
        op: BinOp,
        rhs: &Expr,
        line: u64,
    ) -> Result<Value, LoxError> {
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;

        let res = match op {
            BinOp::Sub => lhs.sub(&rhs),
            BinOp::Div => lhs.div(&rhs),
            BinOp::Mul => lhs.mul(&rhs),
            BinOp::Add => lhs.add(&rhs),
            BinOp::Gt => lhs.gt(&rhs),
            BinOp::GtEq => lhs.ge(&rhs),
            BinOp::Lt => lhs.lt(&rhs),
            BinOp::LtEq => lhs.le(&rhs),
            BinOp::Eq => Ok(Value::from(lhs.eq(&rhs))),
            BinOp::NotEq => Ok(Value::from(lhs.ne(&rhs))),
        };

        res.map_err(|e| LoxError::Runtime(e.to_string(), line))
    }

    fn call(&mut self, callee: &Expr, args: &[Expr], line: u64) -> Result<Value, LoxError> {
        if let Value::Call(fun) = self.evaluate(callee)? {
            let mut values = vec![];
            for arg in args {
                values.push(self.evaluate(arg)?)
            }

            if values.len() != fun.arity() {
                let msg = format!(
                    "Expected {} arguments but got {}.",
                    fun.arity(),
                    values.len()
                );
                return Err(LoxError::Runtime(msg, line));
            }

            fun.call(self, values)
        } else {
            let msg = "Can only call functions and classes.";
            Err(LoxError::Runtime(msg.into(), line))
        }
    }

    fn logical_expr(
        &mut self,
        lhs: &Expr,
        kw: Keyword,
        rhs: &Expr,
        _line: u64,
    ) -> Result<Value, LoxError> {
        let lhs = self.evaluate(lhs)?;

        match kw {
            Keyword::Or if lhs.is_truthy() => Ok(lhs),
            Keyword::And if !lhs.is_truthy() => Ok(lhs),
            _ => self.evaluate(rhs),
        }
    }

    fn unary_expr(&mut self, op: UnOp, rhs: &Expr, line: u64) -> Result<Value, LoxError> {
        let val = self.evaluate(rhs)?;

        let res = match op {
            UnOp::Neg => val.neg(),
            UnOp::Not => Ok(Value::from(!val.is_truthy())),
        };

        res.map_err(|e| LoxError::Runtime(e.to_string(), line))
    }
}

impl ExprVisitor<Result<Value, LoxError>> for Interpreter {
    fn visit_expr(&mut self, e: &Expr) -> Result<Value, LoxError> {
        match e {
            Expr::Assign(name, expr, line) => self.assign_expr(name, expr, *line),
            Expr::Binary { lhs, op, rhs, line } => self.binary_expr(lhs, *op, rhs, *line),
            Expr::Call { callee, args, line } => self.call(callee, args.as_slice(), *line),
            Expr::Grouping(expr) => self.visit_expr(expr),
            Expr::Literal(lit) => Ok(Value::from(lit)),
            Expr::Logical { lhs, kw, rhs, line } => self.logical_expr(lhs, *kw, rhs, *line),
            Expr::Unary(op, expr, line) => self.unary_expr(*op, expr, *line),
            Expr::Variable(name, line) => self
                .env
                .borrow_mut()
                .get(name)
                .map_err(|e| LoxError::Runtime(e, *line)),
        }
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), LoxError>;

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Self::Output {
        self.execute_block(
            &stmts,
            Rc::new(RefCell::new(Env::with_environment(self.env.clone()))),
        )
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Self::Output {
        self.evaluate(expr).map(|_| ())
    }

    fn visit_function_stmt(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        _line: u64,
    ) -> Self::Output {
        let fun = Func::new(name, params, body, self.env.clone());

        self.env
            .borrow_mut()
            .define(name, Value::Call(Rc::new(fun)));

        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then: &Stmt,
        r#else: Option<&Stmt>,
    ) -> Self::Output {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then)?
        } else if let Some(stmt) = r#else {
            self.execute(stmt)?
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Output {
        self.evaluate(expr).map(|val| println!("{}", val))
    }

    fn visit_return_stmt(&mut self, expr: Option<&Expr>, _line: u64) -> Self::Output {
        let val = match expr {
            Some(exp) => self.evaluate(exp)?,
            None => Value::Nil,
        };

        Err(LoxError::Return(val))
    }

    fn visit_var_stmt(&mut self, name: &str, initializer: Option<&Expr>) -> Self::Output {
        let val = if let Some(v) = initializer {
            self.evaluate(&v)?
        } else {
            Value::Nil
        };

        self.env.borrow_mut().define(&name, val);

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?
        }

        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
