#![allow(unused)]

use std::fmt;

pub(crate) type Result<T = ()> = std::result::Result<T, LoxError>;

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
