use std::convert::Infallible;
use crate::buffer::Buffer;
use crate::tokens;
use crate::tokens::{Array, Boolean, LeftBrace, Number, Object, Pair, RightBrace};
use crate::tokens::Node;
use crate::tokens::Offset;
use crate::tokens::Value;
use crate::width::Width;

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

pub struct Parser<'a> {
    content: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
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