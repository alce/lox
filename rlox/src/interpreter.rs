use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

use crate::ast::{BinOp, Expr, Stmt, UnOp};
use crate::value::Value;
use crate::visitor::{ExprVisitor, StmtVisitor};
use crate::LoxError;

pub struct Interpreter {
    env: Rc<RefCell<Env>>,
}

#[derive(Debug)]
struct Env {
    enclosing: Option<Rc<RefCell<Env>>>,
    values: HashMap<String, Value>,
}

impl Env {
    fn new() -> Self {
        Env {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    fn with_environment(env: Rc<RefCell<Env>>) -> Self {
        Env {
            values: HashMap::new(),
            enclosing: Some(env),
        }
    }

    fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    fn get(&mut self, name: &str) -> Result<Value, String> {
        if self.values.contains_key(name) {
            Ok(self.values.get(name).cloned().unwrap())
        } else if let Some(enc) = &mut self.enclosing {
            enc.borrow_mut().get(name)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }

    fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(v) = self.values.get_mut(name) {
            *v = value;
            Ok(())
        } else if let Some(enc) = &mut self.enclosing {
            enc.borrow_mut().assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Rc::new(RefCell::new(Env::new())),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in stmts {
            self.execute(&stmt)?
        }

        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        self.visit_stmt(stmt)
    }

    fn execute_block(&mut self, stmts: &[Stmt], env: Rc<RefCell<Env>>) -> Result<(), LoxError> {
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

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError> {
        self.visit_expr(expr)
    }

    fn unary_expr(&mut self, op: UnOp, rhs: &Expr, line: u64) -> Result<Value, LoxError> {
        let val = self.evaluate(rhs)?;

        let res = match op {
            UnOp::Neg => val.neg(),
            UnOp::Not => val.not(),
        };

        res.map_err(|e| LoxError::Runtime(e.to_string(), line))
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
}

impl ExprVisitor<Result<Value, LoxError>> for Interpreter {
    fn visit_expr(&mut self, e: &Expr) -> Result<Value, LoxError> {
        match e {
            Expr::Literal(lit) => Ok(Value::from(lit)),
            Expr::Grouping(expr) => self.visit_expr(expr),
            Expr::Unary(op, expr, line) => self.unary_expr(*op, expr, *line),
            Expr::Binary { lhs, op, rhs, line } => self.binary_expr(lhs, *op, rhs, *line),
            Expr::Variable(name, line) => self
                .env
                .borrow_mut()
                .get(name)
                .map_err(|e| LoxError::Runtime(e, *line)),
            Expr::Assign(name, expr, line) => self.assign_expr(name, expr, *line),
        }
    }
}

impl StmtVisitor<Result<(), LoxError>> for Interpreter {
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        match stmt {
            Stmt::Expr(expr) => self.evaluate(expr).map(|_| ()),
            Stmt::Print(expr) => self.evaluate(expr).map(|val| println!("{}", val)),
            Stmt::Var(name, value) => {
                let val = if let Some(v) = value {
                    self.evaluate(v)?
                } else {
                    Value::Nil
                };

                Ok(self.env.borrow_mut().define(name, val))
            }
            Stmt::Block(stmts) => self.execute_block(
                stmts,
                Rc::new(RefCell::new(Env::with_environment(self.env.clone()))),
            ),
        }
    }
}
