use std::time::SystemTime;

use crate::value::{Callable, Value};
use crate::{Interpreter, LoxError};

#[derive(Debug)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Result<Value, LoxError> {
        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("There is no time")
            .as_secs();

        Ok(Value::Num(t as f64))
    }
}
