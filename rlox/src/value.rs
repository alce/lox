use std::cmp::Ordering;
use std::fmt::{self, Write};

use crate::ast::Lit;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
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

    pub fn not(self) -> Result<Value, &'static str> {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Ok(Value::Bool(true)),
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
            Value::Str(s) => f.write_str(s),
            Value::Num(n) => {
                if *n == 0.0 && n.is_sign_negative() {
                    f.write_char('-')?;
                }
                write!(f, "{}", n)
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
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
