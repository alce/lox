use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::env::Env;
use crate::value::{Callable, Value};
use crate::{Interpreter, LoxError};

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    params: Vec<String>,
    body: Vec<Stmt>,
}

impl Func {
    pub fn new(params: &[String], body: &[Stmt]) -> Self {
        Func {
            params: params.to_vec(),
            body: body.to_vec(),
        }
    }
}

impl Callable for Func {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, LoxError> {
        let mut env = Env::with_environment(interpreter.globals());

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
