use std::cmp::Ordering;
use std::fmt::{self, Debug, Write};
use std::rc::Rc;

use crate::ast::Lit;
use crate::{Interpreter, LoxError};

pub trait Callable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, LoxError>;
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Call(Rc<dyn Callable>),
    Num(f64),
    Str(String),
}

const NUMBERS_OR_STRINGS: &str = "Operands must be two numbers or two strings.";
const TWO_NUMBERS: &str = "Operands must be numbers.";
const ONE_NUMBER: &str = "Operand must be a number.";

impl Value {
    pub fn add(&self, other: &Self) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
            _ => Err(NUMBERS_OR_STRINGS),
        }
    }

    pub fn sub(&self, other: &Self) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a - b)),
            _ => Err(TWO_NUMBERS),
        }
    }

    pub fn div(&self, other: &Self) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a / b)),
            _ => Err(TWO_NUMBERS),
        }
    }

    pub fn mul(&self, other: &Self) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a * b)),
            _ => Err(TWO_NUMBERS),
        }
    }

    pub fn neg(self) -> Result<Value, &'static str> {
        match self {
            Value::Num(a) => Ok(Value::Num(-a)),
            _ => Err(ONE_NUMBER),
        }
    }

    pub fn lt(&self, other: &Self) -> Result<Value, &'static str> {
        Ok(matches!(self.cmp(other)?, Some(Ordering::Less)).into())
    }

    pub fn le(&self, other: &Self) -> Result<Value, &'static str> {
        Ok(matches!(
            self.cmp(other)?,
            Some(Ordering::Less) | Some(Ordering::Equal)
        )
        .into())
    }

    pub fn gt(&self, other: &Self) -> Result<Value, &'static str> {
        Ok(matches!(self.cmp(other)?, Some(Ordering::Greater)).into())
    }

    pub fn ge(&self, other: &Self) -> Result<Value, &'static str> {
        Ok(matches!(
            self.cmp(other)?,
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
        .into())
    }

    fn cmp(&self, other: &Self) -> Result<Option<Ordering>, &'static str> {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(a.partial_cmp(b)),
            _ => Err(TWO_NUMBERS),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Call(_) => f.write_str("function"),
            Value::Nil => f.write_str("nil"),
            Value::Num(n) => {
                if *n == 0.0 && n.is_sign_negative() {
                    f.write_char('-')?;
                }
                write!(f, "{}", n)
            }
            Value::Str(s) => f.write_str(s),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a.eq(b),
            (Value::Nil, Value::Nil) => true,
            (Value::Num(a), Value::Num(b)) => a.eq(b),
            (Value::Str(a), Value::Str(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<&Lit> for Value {
    fn from(lit: &Lit) -> Self {
        match lit {
            Lit::Num(v) => Value::Num(*v),
            Lit::Str(v) => Value::Str(v.clone()),
            Lit::Bool(v) => Value::Bool(*v),
            Lit::Nil => Value::Nil,
        }
    }
}
