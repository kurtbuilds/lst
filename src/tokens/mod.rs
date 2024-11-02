use crate::Linearize;

mod pair;
mod object;
mod item;
mod array;
mod value;
mod offset;
mod node;

pub use pair::*;
pub use object::*;
pub use item::*;
pub use array::*;
pub use offset::*;
pub use value::*;
pub use node::*;

macro_rules! define_token {
    ($name:ident, $char:expr) => {
        #[derive(Debug)]
        pub struct $name;

        impl $crate::TryParse for $name {
            type Error = $crate::parse::ParseError;
            fn try_parse(mut content: $crate::Buffer<'_>) -> Result<Self, $crate::parse::ParseError> {
                if !content.starts_with($char) {
                    return Err($crate::parse::ParseError::new(concat!(
                        "Expected '",
                        stringify!($char),
                        "'"
                    )));
                }
                // content.slice_start(1);
                Ok(Self)
            }
        }

        impl $crate::Width for $name {
            fn width(&self) -> u32 {
                1
            }
        }

        impl $crate::Linearize for $name {
            fn linearize<'a>(&self, source: &'a str, buf: &mut $crate::LinearizeBuffer<'a>) {
                buf.push((&$name as &dyn std::fmt::Debug, source));
            }
        }
    };
}

#[derive(Debug)]
pub struct Boolean;

define_token!(Colon, ':');
define_token!(LeftBrace, '{');
define_token!(RightBrace, '}');
define_token!(LeftBracket, '[');
define_token!(RightBracket, ']');
define_token!(Comma, ',');

#[derive(Debug)]
pub struct Trivia;
#[derive(Debug)]
pub struct String;
#[derive(Debug)]
pub struct Invalid(crate::parse::ParseError);
#[derive(Debug)]
pub struct Number;
