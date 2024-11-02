use crate::tokens;
use crate::parse::ParseError;
use crate::tokens::{Array, Boolean, Number, Object};

impl Value {
    pub fn has_error(&self) -> bool {
        match &self {
            Value::Invalid(_) => true,
            Value::Object(o) => {
                for pair in &o.pairs {
                    if pair.value.1 .1.has_error() {
                        return true;
                    }
                }
                false
            }
            Value::Array(a) => {
                for value in &a.values {
                    if value.item.1 .1.has_error() {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Boolean(Boolean),
    String(tokens::String),
    Number(Number),
    Object(Object),
    Array(Array),
    Invalid(ParseError),
}