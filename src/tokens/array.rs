use crate::{Linearize, LinearizeBuffer};
use crate::buffer::Buffer;
use crate::parse::{ParseError, TryParse};
use crate::tokens::item::Item;
use crate::tokens::{LeftBracket, RightBracket};
use crate::tokens::node::Node;
use crate::tokens::offset::Offset;
use crate::width::Width;

#[derive(Debug)]
pub struct Array {
    pub left_bracket: Offset<LeftBracket>,
    pub values: Vec<Item>,
    pub right_bracket: Offset<RightBracket>,
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

impl TryParse for Node<Array> {
    type Error = ParseError;

    fn try_parse(mut content: Buffer<'_>) -> Result<Self, Self::Error> {
        let left_bracket = Offset::<LeftBracket>::try_parse(content.offset(0))?;
        let mut values: Vec<Item> = Vec::new();
        let mut offset = left_bracket.width();
        let right_bracket = loop {
            if (content.len() as u32) <= offset {
                return Err(ParseError::new("Encountered array without closing ["));
            }
            if let Ok(right_bracket) = Offset::<RightBracket>::try_parse(content.offset(offset)) {
                offset += right_bracket.width();
                break right_bracket;
            }
            let v = Item::try_parse(content.offset(offset))?;
            if let Some(last) = values.last() {
                if last.comma.is_none() {
                    return Err(ParseError::new("Encountered array items not separated by comma"));
                }
            }
            offset += v.width();
            // let mut buf = Vec::new();
            // v.linearize(&content.content[(offset - v.width()) as usize..offset as usize], &mut buf);
            // dbg!("parsed item", buf, offset);
            values.push(v);
        };
        let array = Array {
            left_bracket,
            values,
            right_bracket,
        };
        // content.slice_start(len);
        Ok(Node(offset, array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let s = "[1, 2, 3]";
        let s = Buffer { content: s };
        let s = Node::<Array>::try_parse(s).unwrap();
        let mut buf = Vec::new();
        s.linearize("[1, 2, 3]", &mut buf);
    }

    #[test]
    fn test_commas_between() {
        let s = "[1 2,]";
        let s = Buffer { content: s };
        let s = Node::<Array>::try_parse(s);
        assert!(s.is_err());
    }
}
