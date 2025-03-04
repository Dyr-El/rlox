use std::ops;

use crate::virtual_machine::InterpretError;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn create_number(value: f64) -> Value {
        Value::Number(value)
    }
    pub fn create_boolean(value: bool) -> Value {
        Value::Boolean(value)
    }
    pub fn create_nil() -> Value {
        Value::Nil
    }
    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }
    pub fn try_as_boolean(&self) -> Result<bool, InterpretError> {
        if let Value::Boolean(value) = self {
            Ok(*value)
        } else {
            Err(InterpretError::RuntimeError)
        }
    }
    pub fn try_as_number(&self) -> Result<f64, InterpretError> {
        if let Value::Number(value) = self {
            Ok(*value)
        } else {
            Err(InterpretError::RuntimeError)
        }
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Value {
        Value::create_number(f)
    }
}

impl Value {
    pub fn print(&self) {
        match self {
            Value::Number(value) => print!("{}", value),
            Value::Boolean(value) => print!("{}", value),
            Value::Nil => print!("nil"),
        }
        
    }
    pub fn is_falsey(&self) -> bool {
        self.is_nil() || (self.is_boolean() && !self.try_as_boolean().unwrap_or(false))
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        if let Ok(value) = self.try_as_number() {
            Value::create_number(-value)
        } else {
            Value::create_nil()
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        if let Ok(value1) = self.try_as_number() {
            if let Ok(value2) = rhs.try_as_number() {
                Value::create_number(value1 + value2)
            } else {
                Value::create_nil()
            }
        } else {
            Value::create_nil()
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Ok(value1) = self.try_as_number() {
            if let Ok(value2) = rhs.try_as_number() {
                Value::create_number(value1 - value2)
            } else {
                Value::create_nil()
            }
        } else {
            Value::create_nil()
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        if let Ok(value1) = self.try_as_number() {
            if let Ok(value2) = rhs.try_as_number() {
                Value::create_number(value1 * value2)
            } else {
                Value::create_nil()
            }
        } else {
            Value::create_nil()
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        if let Ok(value1) = self.try_as_number() {
            if let Ok(value2) = rhs.try_as_number() {
                Value::create_number(value1 / value2)
            } else {
                Value::create_nil()
            }
        } else {
            Value::create_nil()
        }
   }
}