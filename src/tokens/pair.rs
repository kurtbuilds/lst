use crate::{tokens, Linearize, LinearizeBuffer};
use crate::buffer::Buffer;
use crate::parse::{ParseError, TryParse};
use crate::tokens::{Colon, Comma};
use crate::tokens::node::Node;
use crate::tokens::offset::Offset;
use crate::tokens::value::Value;
use crate::width::Width;

#[derive(Debug)]
pub struct Pair {
    pub key: Offset<Node<tokens::String>>,
    pub colon: Offset<Colon>,
    pub value: Offset<Node<Value>>,
    pub comma: Offset<Option<Comma>>,
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