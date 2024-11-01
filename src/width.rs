use crate::{tokens, Node, Offset};

impl<T: Width> Width for Offset<T> {
    fn width(&self) -> u32 {
        self.0 + self.1.width()
    }
}

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

impl<T> Width for Node<T> {
    fn width(&self) -> u32 {
        self.0
    }
}
