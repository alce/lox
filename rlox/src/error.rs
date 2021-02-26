use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    Compile(String),
    Runtime(String, u64),
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
