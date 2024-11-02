use crate::{Linearize, LinearizeBuffer};
use crate::buffer::Buffer;
use crate::parse::{ParseError, TryParse};
use crate::tokens::Comma;
use crate::tokens::node::Node;
use crate::tokens::offset::Offset;
use crate::tokens::value::Value;
use crate::width::Width;

#[derive(Debug)]
pub struct Item {
    pub item: Offset<Node<Value>>,
    pub comma: Offset<Option<Comma>>,
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
        let offset = (self.item.width() + self.comma.0) as usize;
        self.comma.linearize(&source[offset..], buf);
    }
}