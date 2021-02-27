use crate::value::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LoxError {
    Compile(String),
    Runtime(String, u64),
    // This variant carries return values
    Return(Value),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::Compile(msg) => write!(f, "{}", msg),
            LoxError::Runtime(msg, ..) => {
                write!(f, "{}", msg)
            }
            LoxError::Return(v) => {
                write!(f, "RET({})", v)
            }
        }
    }
}

impl std::error::Error for LoxError {}
