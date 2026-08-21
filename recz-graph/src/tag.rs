use owo_colors::OwoColorize;
use recz_adt::Legible;

/// Represents tags in tagged NFA/DFA.
///
/// In practice, the tags are converted into actions during NFA/DFA execution,
/// co you cann look at them as instruction of a NFA/DFA virtual machine.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tag {
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

use Tag::*;

impl Tag {
    /// Returns the group label associated with this tag, if any.
    ///
    /// For now the label is a `u32` value.
    pub fn group_label(&self) -> Option<u32> {
        match self {
            OpenGroup(group_id) => Some(*group_id),
            CloseGroup(group_id) => Some(*group_id),
            DeleteGroup(group_id) => Some(*group_id),
            _ => None,
        }
    }

    pub(crate) fn fmt(&self, f: &mut std::fmt::Formatter<'_>, colored: bool) -> std::fmt::Result {
        if colored {
            match self {
                OpenGroup(group_id) => {
                    write!(f, "{}{}", "+g".bright_blue(), group_id.bright_blue())
                }
                CloseGroup(group_id) => {
                    write!(f, "{}{}", "-g".bright_blue(), group_id.bright_blue())
                }
                DeleteGroup(group_id) => {
                    write!(f, "{}{}", "!g".bright_blue(), group_id.bright_blue())
                }
                StartOfInput => write!(f, "{}", "^".bright_blue()),
                EndOfInput => write!(f, "{}", "$".bright_blue()),
                WordBoundary => write!(f, "{}", "\\b".bright_blue()),
            }
        } else {
            match self {
                OpenGroup(group_id) => write!(f, "+g{group_id}"),
                CloseGroup(group_id) => write!(f, "-g{group_id}"),
                DeleteGroup(group_id) => write!(f, "!g{group_id}"),
                StartOfInput => write!(f, "^"),
                EndOfInput => write!(f, "$"),
                WordBoundary => write!(f, "\\b"),
            }
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
