#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag(usize);

impl Tag {
    pub(crate) fn new(id: usize) -> Self {
        Self(id)
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.0
    }

    pub fn pos_inst(&self) -> Inst {
        Inst::PositiveTag(*self)
    }

    pub fn neg_inst(&self) -> Inst {
        Inst::NegativeTag(*self)
    }
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl std::fmt::$trait for Tag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "t{}", self.0)
            }
        }
    };
}

impl_fmt!(std::fmt::Debug);
impl_fmt!(std::fmt::Display);

#[derive(Debug, Clone)]
pub struct Group {
    label: String,
    open_tag: Tag,
    close_tag: Tag,
}

impl Group {
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

/// Instruction represents the actions that can be performed during a transition
/// step.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Inst {
    /// Non instruction
    Nop,

    /// Store the current position for the corresponding tag.
    PositiveTag(Tag),

    /// Invalidate the specified tag
    NegativeTag(Tag),
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl std::fmt::$trait for Inst {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Inst::Nop => f.write_str("nop"),
                    Inst::PositiveTag(tag) => write!(f, "+{tag}"),
                    Inst::NegativeTag(tag) => write!(f, "-{tag}"),
                }
            }
        }
    };
}

impl_fmt!(std::fmt::Display);
impl_fmt!(std::fmt::Debug);
