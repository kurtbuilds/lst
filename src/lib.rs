#![allow(unused)]

use std::{any::Any, fmt::Debug};
use buffer::Buffer;
use parse::TryParse;
use source::Source;
use tokens::Node;
use tokens::Offset;
use tokens::Value;
use width::Width;
mod tokens;
mod width;
mod source;
mod buffer;
mod parse;


pub fn parse(content: String) -> Source {
    let buf = Buffer { content: &content };
    let dom = Offset::<Node<Value>>::try_parse(buf).expect("error is infallible");
    Source { content, dom }
}

pub type LinearizeBuffer<'a> = Vec<(&'a dyn Debug, &'a str)>;

pub trait Linearize {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>);
}

impl<T: Linearize> Linearize for Option<T> {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        if let Some(v) = self {
            v.linearize(source, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let s = "//hello\n{\n\n";
        let s = parse(s.to_string());
        assert!(s.has_error());
        let s = s.linearize_tokens();
        assert_eq!(s, vec!["//hello\n", "{\n\n"]);

        let s = "//hello\n{\n\n}";
        let s = parse(s.to_string());
        assert!(!s.has_error());
        let s = s.linearize_tokens();
        assert_eq!(s, vec!["//hello\n", "{", "\n\n", "}"]);
    }

    #[test]
    fn test_multi() {
        let s = include_str!("../tests/data/multi.json");
        let s = parse(s.to_string());
        assert!(!s.has_error());
        let s = s.linearize_tokens();
        let tt: Vec<&str> = vec![];
        assert_eq!(s, tt);
    }
}
