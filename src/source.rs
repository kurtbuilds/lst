use crate::{Linearize, LinearizeBuffer};
use crate::tokens::Node;
use crate::tokens::Offset;
use crate::tokens::Value;

/// Rules
/// 1. The parent is responsible for "cleaning up" the surroundings. i.e. having offset<node<_>>
/// 2. Every child lives inside Offset
/// 3. If a child is variable length, it lives inside Node.
#[derive(Debug)]
pub struct Source {
    pub content: String,
    pub dom: Offset<Node<Value>>,
}

impl Source {
    pub fn linearize(&self) -> LinearizeBuffer {
        let mut result = Vec::new();
        self.dom.linearize(self.content.as_str(), &mut result);
        result
    }

    pub fn linearize_tokens(&self) -> Vec<&str> {
        let mut result = Vec::new();
        self.dom.linearize(self.content.as_str(), &mut result);
        result.into_iter().map(|(_, token)| token).collect()
    }

    pub fn has_error(&self) -> bool {
        self.dom.1 .1.has_error()
    }
}