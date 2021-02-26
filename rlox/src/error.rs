use std::fmt;

use crate::interpreter::RealRuntimeError;
use crate::parser::ParseError;

#[derive(Debug)]
pub enum LoxError {
    Compile(String),
    Runtime(String, u64),
}

impl From<ParseError> for LoxError {
    fn from(e: ParseError) -> Self {
        match e {
            ParseError::Parse(msg) => LoxError::Compile(msg),
            ParseError::Syntax(msg) => LoxError::Compile(msg),
        }
    }
}

impl From<RealRuntimeError> for LoxError {
    fn from(e: RealRuntimeError) -> Self {
        LoxError::Runtime(e.0, e.1)
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::Compile(msg) => write!(f, "{}", msg),
            LoxError::Runtime(msg, ..) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for LoxError {}
