#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag(u32);

impl Tag {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }
}

pub struct Group(Tag, Tag);

impl Group {
    #[inline]
    pub fn new(open_tag: Tag, close_tag: Tag) -> Self {
        Self(open_tag, close_tag)
    }

    #[inline]
    pub fn open_tag(&self) -> Tag {
        self.0
    }

    #[inline]
    pub fn close_tag(&self) -> Tag {
        self.1
    }
}
