use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

#[derive(Debug)]
pub struct Env {
    enclosing: Option<Rc<RefCell<Env>>>,
    values: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_environment(env: Rc<RefCell<Env>>) -> Self {
        Env {
            values: HashMap::new(),
            enclosing: Some(env),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&mut self, name: &str) -> Result<Value, String> {
        if self.values.contains_key(name) {
            Ok(self.values.get(name).cloned().unwrap())
        } else if let Some(enc) = &mut self.enclosing {
            enc.borrow_mut().get(name)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(v) = self.values.get_mut(name) {
            *v = value;
            Ok(())
        } else if let Some(enc) = &mut self.enclosing {
            enc.borrow_mut().assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        //
        Env::new()
    }
}
