use crate::tokens::Node;
use crate::tokens::Offset;


pub trait Width {
    fn width(&self) -> u32;
}

impl Width for char {
    fn width(&self) -> u32 {
        self.len_utf8() as u32
    }
}

impl<T: Width> Width for Option<T> {
    fn width(&self) -> u32 {
        self.as_ref().map(|t| t.width()).unwrap_or(0)
    }
}

