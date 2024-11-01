#![allow(unused)]

use std::{any::Any, convert::Infallible, fmt::Debug};

use tokens::{
    Boolean, Colon, Comma, LeftBrace, LeftBracket, Number, RightBrace, RightBracket, Trivia,
};
use width::Width;
mod tokens;
mod width;

/// Rules
/// 1. The parent is responsible for "cleaning up" the surroundings. i.e. having offset<node<_>>
/// 2. Every child lives inside Offset
/// 3. If a child is variable length, it lives inside Node.
#[derive(Debug)]
pub struct Source {
    pub content: std::string::String,
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

#[derive(Debug, Clone)]
pub struct Buffer<'a> {
    content: &'a str,
}

impl Buffer<'_> {
    pub fn offset(&self, offset: u32) -> Self {
        Self {
            content: &self.content[offset as usize..],
        }
    }

    pub fn cutoff(&self, len: u32) -> Self {
        Self {
            content: &self.content[..len as usize],
        }
    }

    // pub fn slice_start(&mut self, len: u32) {
    //     self.content = &self.content[len as usize..];
    // }
}

impl std::ops::Deref for Buffer<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content
    }
}

pub trait TryParse: Sized {
    type Error;
    fn try_parse(content: Buffer<'_>) -> Result<Self, Self::Error>;
}

impl<T: TryParse> TryParse for Option<T> {
    type Error = Infallible;
    fn try_parse(content: Buffer<'_>) -> Result<Self, Infallible> {
        Ok(T::try_parse(content).ok())
    }
}

impl TryParse for Node<tokens::String> {
    type Error = ParseError;
    fn try_parse(mut content: Buffer<'_>) -> Result<Self, ParseError> {
        let mut chars = content.chars();
        let Some(c) = chars.next() else {
            return Err(ParseError::new("String must contain a character"));
        };
        if c != '"' {
            return Err(ParseError::new("String must start with '\"'"));
        }
        let mut len = c.width();
        while let Some(c) = chars.next() {
            len += c.width();
            if c == '\\' {
                chars.next();
                len += c.width();
            } else if c == '"' {
                // content.slice_start(len);
                return Ok(Node(len, tokens::String));
            }
        }
        Err(ParseError::new("String must end with '\"'"))
    }
}

impl TryParse for Node<Number> {
    type Error = ParseError;
    fn try_parse(mut content: Buffer<'_>) -> Result<Self, ParseError> {
        let mut chars = content.chars();
        let Some(c) = chars.next() else {
            return Err(ParseError::new("Number must contain at least one digit"));
        };
        let mut len = c.width();
        if c < '0' || c > '9' {
            return Err(ParseError::new("Number must start with a digit"));
        }
        let leading_zero = c == '0';
        let mut dot_position = -1;
        while let Some(c) = chars.next() {
            len += c.width();
            if c == '.' {
                if dot_position > -1 {
                    return Err(ParseError::new("Encountered a second . in a number"));
                }
                dot_position = len as i32 - 1;
            } else if !c.is_digit(10) {
                len -= c.width();
                break;
            }
        }
        if leading_zero && (dot_position == -1 || dot_position > 1) {
            return Err(ParseError::new("Number cannot have leading zeros"));
        }
        // content.slice_start(len);
        Ok(Node(len, Number))
    }
}

impl TryParse for Node<Boolean> {
    type Error = ParseError;
    fn try_parse(mut content: Buffer<'_>) -> Result<Self, ParseError> {
        if content.starts_with("true") {
            // content.slice_start(4);
            Ok(Node(4, Boolean))
        } else if content.starts_with("false") {
            // content.slice_start(5);
            Ok(Node(5, Boolean))
        } else {
            Err(ParseError::new("Boolean must be either true or false"))
        }
    }
}

impl TryParse for Node<Array> {
    type Error = ParseError;

    fn try_parse(mut content: Buffer<'_>) -> Result<Self, Self::Error> {
        let left_bracket = Offset::<LeftBracket>::try_parse(content.offset(0))?;
        let mut values = Vec::new();
        let mut offset = left_bracket.width();
        while let Ok(v) = Item::try_parse(content.offset(offset)) {
            offset += v.width();
            values.push(v);
        }
        let right_bracket = Offset::<RightBracket>::try_parse(content.offset(offset))?;
        offset += right_bracket.width();
        let array = Array {
            left_bracket,
            values,
            right_bracket,
        };
        // content.slice_start(len);
        Ok(Node(offset, array))
    }
}
#[derive(Debug)]
pub struct Item {
    item: Offset<Node<Value>>,
    comma: Offset<Option<Comma>>,
}

impl TryParse for Item {
    type Error = ParseError;

    fn try_parse(content: Buffer<'_>) -> Result<Self, Self::Error> {
        let item = Offset::<Node<Value>>::try_parse(content.offset(0)).unwrap();
        let comma = Offset::<Option<Comma>>::try_parse(content.offset(item.width())).unwrap();
        Ok(Item { item, comma })
    }
}

impl Width for Item {
    fn width(&self) -> u32 {
        self.item.width() + self.comma.width()
    }
}

impl Linearize for Item {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        self.item.linearize(source, buf);
        self.comma.linearize(source, buf);
    }
}

#[derive(Debug)]
pub struct Array {
    left_bracket: Offset<LeftBracket>,
    values: Vec<Item>,
    right_bracket: Offset<RightBracket>,
}

impl Linearize for Array {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        self.left_bracket.linearize(source, buf);
        for v in &self.values {
            v.linearize(source, buf);
        }
        self.right_bracket.linearize(source, buf);
    }
}

#[derive(Debug)]
pub struct Pair {
    key: Offset<Node<tokens::String>>,
    colon: Offset<Colon>,
    value: Offset<Node<Value>>,
    comma: Offset<Option<Comma>>,
}

impl TryParse for Pair {
    type Error = ParseError;

    fn try_parse(content: Buffer<'_>) -> Result<Self, Self::Error> {
        let key = Offset::<Node<tokens::String>>::try_parse(content.offset(0))?;
        let mut offset = key.width();
        let colon = Offset::<Colon>::try_parse(content.offset(offset))?;
        offset += colon.width();
        let value = Offset::<Node<Value>>::try_parse(content.offset(offset)).unwrap();
        offset += value.width();
        let comma = Offset::<Option<Comma>>::try_parse(content.offset(offset)).unwrap();
        Ok(Pair {
            key,
            colon,
            value,
            comma,
        })
    }
}

impl Linearize for Pair {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        self.key.linearize(source, buf);
        self.colon.linearize(source, buf);
        self.value.linearize(source, buf);
        self.comma.linearize(source, buf);
    }
}

impl Width for Pair {
    fn width(&self) -> u32 {
        self.key.width() + self.colon.width() + self.value.width() + self.comma.width()
    }
}

#[derive(Debug)]
pub struct Object {
    left_brace: Offset<LeftBrace>,
    pub pairs: Vec<Pair>,
    right_brace: Offset<RightBrace>,
}

impl Linearize for Object {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        let mut offset = 0;
        self.left_brace.linearize(&source[offset..offset + 1], buf);
        offset += self.left_brace.width() as usize;
        for p in &self.pairs {
            p.linearize(&source[offset..offset + p.width() as usize], buf);
            offset += p.width() as usize;
        }
        dbg!(source, offset);
        self.right_brace.linearize(&source[offset..], buf);
    }
}

pub struct Parser<'a> {
    content: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
}

impl TryParse for Node<Object> {
    type Error = ParseError;

    fn try_parse(content: Buffer<'_>) -> Result<Self, Self::Error> {
        let left_brace = Offset::<LeftBrace>::try_parse(content.offset(0))?;
        let mut pairs = Vec::new();
        let mut offset = left_brace.width();
        while let Ok(p) = Pair::try_parse(content.offset(offset)) {
            offset += p.width();
            pairs.push(p);
        }
        let right_brace = Offset::<RightBrace>::try_parse(content.offset(offset))?;
        offset += right_brace.width();
        let object = Object {
            left_brace,
            pairs,
            right_brace,
        };
        // content.slice_start(len);
        Ok(Node(offset, object))
    }
}

// pub trait Parse: Sized {
//     fn parse(content: &mut Buffer<'_>) -> Self;
// }
#[derive(Debug)]
pub struct ParseError {
    message: std::borrow::Cow<'static, str>,
}

impl ParseError {
    pub fn new(message: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub struct Offset<T>(u32, T);

impl<T: TryParse> TryParse for Offset<T> {
    type Error = T::Error;
    fn try_parse(mut content: Buffer<'_>) -> Result<Self, Self::Error> {
        let mut offset = 0;
        let mut chars = content.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next().unwrap();
                offset += c.width();
            } else if c == '/' && chars.peek().as_deref() == Some(&'/') {
                while let Some(c) = chars.next() {
                    offset += c.width();
                    if c == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        dbg!(&content[offset as usize..]);
        let res = match T::try_parse(content.offset(offset)) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        // content.slice_start(offset);
        Ok(Offset(offset, res))
    }
}

impl<T: Linearize> Linearize for Offset<T> {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        dbg!(source, self.0);
        if self.0 > 0 {
            let slice = &source[0..self.0 as usize];
            buf.push((&Trivia as &dyn Debug, slice));
        }
        self.1.linearize(&source[self.0 as usize..], buf);
    }
}

#[derive(Debug)]
pub struct Node<T>(u32, T);

impl<T> Node<T> {
    pub fn map_value<U>(self, m: impl Fn(T) -> U) -> Node<U> {
        Node(self.0, m(self.1))
    }
}

impl TryParse for Node<Value> {
    type Error = Infallible;
    fn try_parse(mut content: Buffer<'_>) -> Result<Self, Self::Error> {
        let mut chars = content.chars();
        let Some(c) = chars.next() else {
            return Ok(Node(0, Value::Invalid(ParseError::new("Empty content"))));
        };
        let len = content.len() as u32;
        let res = match c {
            '"' => Node::<tokens::String>::try_parse(content).map(|n| n.map_value(Value::String)),
            '0'..='9' => Node::<Number>::try_parse(content).map(|n| n.map_value(Value::Number)),
            't' | 'f' => Node::<Boolean>::try_parse(content).map(|n| n.map_value(Value::Boolean)),
            '{' => Node::<Object>::try_parse(content).map(|n| n.map_value(Value::Object)),
            '[' => Node::<Array>::try_parse(content).map(|n| n.map_value(Value::Array)),
            _ => {
                return Ok(Node(len, Value::Invalid(ParseError::new("Invalid value"))));
            }
        };
        Ok(res.unwrap_or_else(|e| Node(len, Value::Invalid(e))))
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

pub fn parse(content: std::string::String) -> Source {
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

impl Linearize for Node<tokens::String> {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        let slice = &source[0..self.0 as usize];
        buf.push((&tokens::String as &dyn Debug, slice));
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
}
