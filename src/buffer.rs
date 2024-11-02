#[derive(Debug, Clone)]
pub struct Buffer<'a> {
    pub content: &'a str,
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