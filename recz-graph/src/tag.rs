use owo_colors::OwoColorize;
use recz_adt::Legible;

/// Represents tags in tagged NFA/DFA.
///
/// In practice, the tags are converted into actions during NFA/DFA execution,
/// co you cann look at them as instruction of a NFA/DFA virtual machine.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag {
    kind: TagKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TagKind {
    /// A tag used to mark the start of a capture group.
    OpenGroup(u32),

    /// A tag used to mark the end of a capture group.
    CloseGroup(u32),

    /// A tag used to mark a group's tags for deletion.
    DeleteGroup(u32),

    /// A tag used to mark zero sized start of input text (or start of line in
    /// multiline mode).
    StartOfInput,

    /// A tag used to mark zero sized end of input text (or end of line in
    /// multiline mode).
    EndOfInput,

    /// A tag used to mark a border between a word and a non-word.
    WordBoundary,
}

use TagKind::*;

impl Tag {
    pub(crate) fn new(kind: TagKind) -> Self {
        Self { kind }
    }

    pub(crate) fn fmt(&self, f: &mut std::fmt::Formatter<'_>, colored: bool) -> std::fmt::Result {
        if colored {
            match self.kind {
                OpenGroup(group_idx) => {
                    write!(f, "{}{}", "+g".bright_blue(), group_idx.bright_blue())
                }
                CloseGroup(group_idx) => {
                    write!(f, "{}{}", "-g".bright_blue(), group_idx.bright_blue())
                }
                DeleteGroup(group_idx) => {
                    write!(f, "{}{}", "!g".bright_blue(), group_idx.bright_blue())
                }
                StartOfInput => write!(f, "{}", "^".bright_blue()),
                EndOfInput => write!(f, "{}", "$".bright_blue()),
                WordBoundary => write!(f, "{}", "\\b".bright_blue()),
            }
        } else {
            match self.kind {
                OpenGroup(group_idx) => write!(f, "+g{group_idx}"),
                CloseGroup(group_idx) => write!(f, "-g{group_idx}"),
                DeleteGroup(group_idx) => write!(f, "!g{group_idx}"),
                StartOfInput => write!(f, "^"),
                EndOfInput => write!(f, "$"),
                WordBoundary => write!(f, "\\b"),
            }
        }
    }
}

impl Tag {
    pub fn kind(&self) -> TagKind {
        self.kind
    }

    /// Returns a `DeleteGroup` tag if the current tag is an `OpenGroup` or
    /// `CloseGroup`.
    pub fn delete_group(&self) -> Option<Tag> {
        match self.kind {
            OpenGroup(idx) => Some(Tag::new(DeleteGroup(idx))),
            CloseGroup(idx) => Some(Tag::new(DeleteGroup(idx))),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, false)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, false)
    }
}

impl Legible for Tag {
    fn legible(&self) -> impl core::fmt::Display {
        self
    }

    fn colored(&self) -> impl core::fmt::Display {
        struct ColoredTag(Tag);
        impl core::fmt::Display for ColoredTag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Tag::fmt(&self.0, f, true)
            }
        }
        ColoredTag(*self)
    }
}
