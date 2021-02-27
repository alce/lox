use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::env::Env;
use crate::value::{Callable, Value};
use crate::{Interpreter, LoxError};

#[derive(Debug, Clone)]
pub struct Func {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Env>>,
}

impl Func {
    pub fn new(name: &str, params: &[String], body: &[Stmt], closure: Rc<RefCell<Env>>) -> Self {
        Func {
            name: name.into(),
            params: params.to_vec(),
            body: body.to_vec(),
            closure,
        }
    }
}

impl Callable for Func {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, LoxError> {
        let mut env = Env::with_environment(self.closure.clone());

        self.params
            .iter()
            .zip(args.iter())
            .for_each(|(name, value)| env.borrow_mut().define(name, value.clone()));

        let res = interpreter.execute_block(&self.body, Rc::new(RefCell::new(env)));

        if let Err(LoxError::Return(val)) = res {
            Ok(val)
        } else {
            Ok(Value::Nil)
        }
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
