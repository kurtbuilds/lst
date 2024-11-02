use std::fmt::Debug;
use crate::{Linearize, LinearizeBuffer};
use crate::buffer::Buffer;
use crate::parse::TryParse;
use crate::tokens::{Node, Trivia};
use crate::width::Width;

#[derive(Debug)]
pub struct Offset<T>(pub u32, pub T);

impl<T> std::ops::Deref for Offset<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T: Width> Width for Offset<T> {
    fn width(&self) -> u32 {
        // println!("{}", self.0);
        self.0 + self.1.width()
    }
}
impl<T: Linearize> Linearize for Offset<T> {
    fn linearize<'a>(&'a self, source: &'a str, buf: &mut LinearizeBuffer<'a>) {
        if self.0 > 0 {
            let slice = &source[0..self.0 as usize];
            buf.push((&Trivia as &dyn Debug, slice));
        }
        self.1.linearize(&source[self.0 as usize..], buf);
    }
}

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
        let res = match T::try_parse(content.offset(offset)) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        // content.slice_start(offset);
        Ok(Offset(offset, res))
    }
}
