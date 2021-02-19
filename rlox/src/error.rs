use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    Compile(String),
    Runtime,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LoxError {}
