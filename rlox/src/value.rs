use std::{fmt, ops};

use crate::ast::Lit;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl ops::Add for Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x + y)),
            (Value::Str(s1), Value::Str(s2)) => Ok(Value::Str(s1 + &s2)),
            _ => Err("Operands must be two numbers or two strings.".into()),
        }
    }
}

impl ops::Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x - y)),
            _ => Err("Operands must be numbers.".into()),
        }
    }
}

impl ops::Div for Value {
    type Output = Result<Value, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x / y)),
            _ => Err("Operands must be numbers.".into()),
        }
    }
}

impl ops::Mul for Value {
    type Output = Result<Value, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x * y)),
            _ => Err("Operands must be numbers.".into()),
        }
    }
}

impl ops::Neg for Value {
    type Output = Result<Value, String>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Num(n) => Ok(Value::Num(-n)),
            _ => Err("Operand must be a number".into()),
        }
    }
}

impl ops::Not for Value {
    type Output = Result<Value, String>;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Ok(Value::Bool(true)),
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
