use crate::tag::Tag;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptureLabel {
    Num(u32),
    Str(Rc<str>),
}

impl core::convert::From<u8> for CaptureLabel {
    fn from(value: u8) -> Self {
        CaptureLabel::Num(value as u32)
    }
}

impl core::convert::From<u16> for CaptureLabel {
    fn from(value: u16) -> Self {
        CaptureLabel::Num(value as u32)
    }
}

impl core::convert::From<u32> for CaptureLabel {
    fn from(value: u32) -> Self {
        CaptureLabel::Num(value)
    }
}

impl core::convert::From<Rc<str>> for CaptureLabel {
    fn from(value: Rc<str>) -> Self {
        CaptureLabel::Str(value)
    }
}

impl core::convert::From<String> for CaptureLabel {
    fn from(value: String) -> Self {
        CaptureLabel::Str(value.into())
    }
}

impl<'a> core::convert::From<&'a str> for CaptureLabel {
    fn from(value: &'a str) -> Self {
        CaptureLabel::Str(value.into())
    }
}

impl equivalent::Equivalent<CaptureGroup> for CaptureLabel {
    fn equivalent(&self, other: &CaptureGroup) -> bool {
        PartialEq::eq(self, &other.label)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureGroup {
    label: CaptureLabel,
    tag_index: u32,
}

impl CaptureGroup {
    pub fn open_tag(&self) -> Tag {
        Tag::OpenGroup(self.tag_index)
    }

    pub fn close_tag(&self) -> Tag {
        Tag::CloseGroup(self.tag_index)
    }

    pub fn delete_tag(&self) -> Tag {
        Tag::DeleteGroup(self.tag_index)
    }
}

impl CaptureGroup {
    pub(crate) fn new(label: CaptureLabel, tag_index: u32) -> Self {
        Self { label, tag_index }
    }
}

impl core::hash::Hash for CaptureGroup {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}
