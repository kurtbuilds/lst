use crate::{Linearize, LinearizeBuffer};
use crate::buffer::Buffer;
use crate::parse::{ParseError, TryParse};
use crate::tokens::{LeftBrace, Node, RightBrace};
use crate::tokens::offset::Offset;
use crate::tokens::pair::Pair;
use crate::width::Width;

#[derive(Debug)]
pub struct Object {
    pub left_brace: Offset<LeftBrace>,
    pub pairs: Vec<Pair>,
    pub right_brace: Offset<RightBrace>,
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

impl Linearize for Object {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        let mut offset = 0;
        self.left_brace.linearize(&source[offset..offset + 1], buf);
        offset += self.left_brace.width() as usize;
        for p in &self.pairs {
            p.linearize(&source[offset..offset + p.width() as usize], buf);
            offset += p.width() as usize;
        }
        // dbg!(source, offset);
        self.right_brace.linearize(&source[offset..], buf);
    }
}