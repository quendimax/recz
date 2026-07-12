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
    pub fn opposite(&self) -> Tag {
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

#[derive(Debug, Clone)]
pub struct Group {
    id: u32,
    label: String,
    open_tag: Tag,
    close_tag: Tag,
}

impl Group {
    pub(crate) fn new(id: u32, label: String) -> Self {
        Self {
            id,
            label,
            open_tag: Tag::Open(id),
            close_tag: Tag::Close(id),
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn open_tag(&self) -> Tag {
        self.open_tag
    }

    #[inline]
    pub fn close_tag(&self) -> Tag {
        self.close_tag
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
