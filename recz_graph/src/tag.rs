use std::rc::Rc;

/// Represents a tag used for marking groups in regexps.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tag {
    /// A tag used to mark the start of a group.
    Open(u32),
    /// A tag used to mark the end of a group.
    Close(u32),
    /// A tag used to mark a group's tags for deletion.
    Delete(u32),
}

impl Tag {
    /// Returns a tag that is a marker for deletion of the group associated with
    /// this tag.
    pub fn deleter(&self) -> Tag {
        match self {
            Self::Open(group_id) => Self::Delete(*group_id),
            Self::Close(group_id) => Self::Delete(*group_id),
            Self::Delete(group_id) => Self::Delete(*group_id),
        }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(group_id) => write!(f, "+t{group_id}"),
            Self::Close(group_id) => write!(f, "-t{group_id}"),
            Self::Delete(group_id) => write!(f, "~t{group_id}"),
        }
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[derive(Clone)]
pub struct Group {
    id: u32,
    label: Rc<str>,
}

impl Group {
    pub(crate) fn new(id: u32, label: Rc<str>) -> Self {
        Self { id, label }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn open_tag(&self) -> Tag {
        Tag::Open(self.id)
    }

    #[inline]
    pub fn close_tag(&self) -> Tag {
        Tag::Close(self.id)
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl std::fmt::Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("id", &self.id)
            .field("label", &self.label)
            .finish()
    }
}
