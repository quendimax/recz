use crate::tag::Tag;
use recz_adt::{ByteIter, Legible, RangeIter, RangeU8, Set, SetU8, Step};
use std::fmt::Write;
use std::iter::Iterator;

/// Edge is a transition from one node to another that contains symbols and
/// tags. The symbols are bytes. If the edge doesn't have any symbols it is
/// treated as an epsilon transition.
///
/// # Implementation
///
/// The struct itself is hold in the heap. The `Edge` is a thin wrapper around
/// reference to the struct. So it can be cheaply copied, and the new copies are
/// just new references to the same data
pub struct Edge<'a>(&'a EdgeInner);

pub(crate) struct EdgeInner {
    symbols: SetU8,
    tags: Set<Tag>,
}

pub(crate) type EdgePtr = core::ptr::NonNull<EdgeInner>;

/// Public API
impl<'a> Edge<'a> {
    /// Checks if these edges are two references to the same edge.
    #[inline]
    pub fn is(self, other: Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }

    #[inline]
    pub fn is_epsilon(&self) -> bool {
        self.0.symbols.is_empty()
    }

    /// Returns an iterator over all symbols (bytes) in this edge.
    pub fn symbols(&self) -> ByteIter {
        self.0.symbols.iter()
    }

    /// Returns iterator over all symbol ranges in this edge instance in
    /// ascendent order.
    pub fn ranges(&self) -> RangeIter {
        self.0.symbols.ranges()
    }

    #[inline]
    pub fn tags(&self) -> impl Iterator<Item = Tag> {
        self.0.tags.iter().copied()
    }

    /// Adds a tag to this edge.
    pub fn add_tag(&self, tag: Tag) {
        self.0.tags.insert(tag);
    }

    /// Adds a symbol to this edge.
    pub fn add_symbol(&self, symbol: u8) {
        self.0.symbols.insert(symbol);
    }

    /// Adds symbols to this edge.
    pub fn add_symbols(&self, symbols: impl Into<SetU8>) {
        self.0.symbols.insert_bytes(symbols);
    }

    /// Returns whether this edge contains the given symbol.
    pub fn contains_symbol(&self, symbol: u8) -> bool {
        self.0.symbols.contains(symbol)
    }

    /// Returns whether this edge contains all the given symbols.
    pub fn contains_symbols(&self, symbols: impl Into<SetU8>) -> bool {
        self.0.symbols.contains_bytes(symbols)
    }

    /// Returns whether this edge contains the given tag.
    pub fn contains_tag(&self, tag: Tag) -> bool {
        self.0.tags.contains(&tag)
    }

    pub fn is_subset(&self, other: Self) -> bool {
        other.is_superset(*self)
    }

    pub fn is_superset(&self, other: Self) -> bool {
        self.0.symbols.is_superset(&other.0.symbols) && self.0.tags.is_superset(&other.0.tags)
    }

    pub fn intersects(&self, other: Self) -> bool {
        !self.0.symbols.is_disjoint(&other.0.symbols) || !self.0.tags.is_disjoint(&other.0.tags)
    }
}

/// Private API
impl<'a> Edge<'a> {
    pub(crate) fn from_ref(edge_ref: &'a EdgeInner) -> Self {
        Self(edge_ref)
    }

    #[inline(always)]
    pub(crate) fn as_ptr(&self) -> EdgePtr {
        EdgePtr::from(self.0)
    }
}

impl Copy for Edge<'_> {}

impl Clone for Edge<'_> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl std::cmp::Eq for Edge<'_> {}

impl std::cmp::PartialEq for Edge<'_> {
    /// Tests equality between symbols only, not instructions.
    fn eq(&self, other: &Self) -> bool {
        self.0.symbols.eq(&other.0.symbols)
    }
}

impl std::fmt::Display for Edge<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // symbols
        f.write_char('[')?;
        if self.is_epsilon() {
            f.write_str("Epsilon")?;
        } else {
            let mut iter = self.ranges();
            let mut range = iter.next();
            while let Some(cur_range) = range {
                if let Some(next_range) = iter.next() {
                    if cur_range.last().steps_between(next_range.start()) == 1 {
                        range = Some(RangeU8::new(cur_range.start(), next_range.last()));
                        continue;
                    } else {
                        std::fmt::Display::fmt(&cur_range.display(), f)?;
                        f.write_str(" | ")?;
                        range = Some(next_range);
                    }
                } else {
                    std::fmt::Display::fmt(&cur_range.display(), f)?;
                    break;
                }
            }
        }
        f.write_char(']')?;

        // tags
        if !self.0.tags.is_empty() {
            f.write_str(" / ")?;
            let mut first_tag = true;
            for tag in self.tags() {
                if first_tag {
                    first_tag = false;
                } else {
                    f.write_char(',')?;
                }
                std::fmt::Display::fmt(&tag, f)?;
            }
        }
        Ok(())
    }
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl std::fmt::$trait for Edge<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_char('[')?;
                if self.is_epsilon() {
                    f.write_str("Epsilon")?;
                } else {
                    let mut first_iter = true;
                    for range in self.ranges() {
                        if first_iter {
                            first_iter = false;
                        } else {
                            f.write_str(" | ")?;
                        }
                        ::std::fmt::$trait::fmt(&range, f)?;
                    }
                }
                f.write_char(']')
            }
        }
    };
}

impl_fmt!(std::fmt::Debug);
impl_fmt!(std::fmt::Binary);
impl_fmt!(std::fmt::Octal);
impl_fmt!(std::fmt::LowerHex);
impl_fmt!(std::fmt::UpperHex);

impl EdgeInner {
    /// Creates a new empty edge.
    #[inline(always)]
    pub(crate) fn new() -> EdgeInner {
        EdgeInner {
            symbols: SetU8::new(),
            tags: Set::new(),
        }
    }
}
