use std::fmt::Debug;
use crate::{tokens, Linearize, LinearizeBuffer};
use crate::tokens::Value;
use crate::width::Width;

#[derive(Debug)]
pub struct Node<T>(pub u32, pub T);

impl<T> std::ops::Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> Node<T> {
    pub fn map_value<U>(self, m: impl Fn(T) -> U) -> Node<U> {
        Node(self.0, m(self.1))
    }
}

impl<T> Width for Node<T> {
    fn width(&self) -> u32 {
        self.0
    }
}

impl Linearize for Node<Value> {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        let slice = &source[0..self.0 as usize];
        match &self.1 {
            Value::String(s) => buf.push((s as &dyn Debug, slice)),
            Value::Number(n) => buf.push((n as &dyn Debug, slice)),
            Value::Boolean(b) => buf.push((b as &dyn Debug, slice)),
            Value::Object(o) => o.linearize(source, buf),
            Value::Array(a) => a.linearize(source, buf),
            Value::Invalid(e) => buf.push((e as &dyn Debug, slice)),
        }
    }
}

impl Linearize for Node<tokens::String> {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        let slice = &source[0..self.0 as usize];
        buf.push((&tokens::String as &dyn Debug, slice));
    }
}