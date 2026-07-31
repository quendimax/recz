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
/// just new references to the same data.
pub struct Edge<'a>(&'a EdgeInner);

pub(crate) struct EdgeInner {
    symbols: SetU8,
    tags: Set<Tag>,
}

/// NullPtr over `EdgeInner`.
pub(crate) type EdgePtr = core::ptr::NonNull<EdgeInner>;

/// Public API
impl<'a> Edge<'a> {
    /// Checks if these edges are two references to the same edge.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_graph::Graph;
    /// let graph = Graph::new();
    /// let node_a = graph.node();
    /// let node_b = graph.node();
    /// assert!(node_a.connect(node_b).is(node_a.connect(node_b)));
    /// assert!(!node_a.connect(node_b).is(node_b.connect(node_a)));
    /// ```
    #[inline]
    pub fn is(self, other: Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }

    /// Checks if this edge is an epsilon edge (contains no symbols).
    ///
    /// A new edge, created by [`Node::connect`], is an epsilon edge. As you add
    /// symbols to the edge, it becomes a non-epsilon edge.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_graph::Graph;
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// assert!(edge.is_epsilon());
    /// edge.add_symbol(3);
    /// assert!(!edge.is_epsilon());
    /// ```
    #[inline]
    pub fn is_epsilon(&self) -> bool {
        self.0.symbols.is_empty()
    }

    /// Adds a tag to this edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_graph::{Graph, Tag};
    ///
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_tag(Tag::OpenGroup(0));
    /// ```
    pub fn add_tag(&self, tag: Tag) {
        self.0.tags.insert(tag);
    }

    /// Adds a symbol to this edge.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_graph::Graph;
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_symbol(3);
    /// ```
    pub fn add_symbol(&self, symbol: u8) {
        self.0.symbols.insert(symbol);
    }

    /// Adds a symbol collection to this edge.
    ///
    /// The `symbols` parameter is a symbol collection that is covertible into a
    /// [`SetU8`]. For now, these are [`RangeU8`], [`RangeInclusive<u8>`],
    /// `[u8]`, or just `u8`.
    ///
    /// ```
    /// # use recz_graph::Graph;
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_symbols(0..=10);
    /// edge.add_symbols([30, 40, 50]);
    /// ```
    pub fn add_symbols(&self, symbols: impl Into<SetU8>) {
        self.0.symbols.insert_bytes(symbols);
    }

    /// Returns whether this edge contains the given symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_graph::Graph;
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_symbols(0..=10);
    /// assert!(edge.contains_symbol(5));
    /// assert!(!edge.contains_symbol(20));
    /// ```
    pub fn contains_symbol(&self, symbol: u8) -> bool {
        self.0.symbols.contains(symbol)
    }

    /// Returns whether this edge contains all the given symbols.
    pub fn contains_symbols(&self, symbols: impl Into<SetU8>) -> bool {
        self.0.symbols.contains_bytes(symbols)
    }

    /// Returns whether this edge contains the given tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_graph::Graph;
    /// use recz_graph::Tag;
    ///
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_tag(Tag::OpenGroup(0));
    ///
    /// assert!(edge.contains_tag(Tag::OpenGroup(0)));
    /// assert!(!edge.contains_tag(Tag::CloseGroup(0)));
    /// ```
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

    /// Returns an iterator over all symbols (bytes) in this edge in ascending
    /// order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_graph::Graph;
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_symbols([1, 3, 4]);
    /// assert_eq!(edge.symbols().collect::<Vec<_>>(), [1, 3, 4]);
    /// ```
    pub fn symbols(&self) -> ByteIter {
        self.0.symbols.iter()
    }

    /// Returns iterator over all symbol ranges in this edge in ascendent order.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::RangeU8;
    /// use recz_graph::Graph;
    ///
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_symbols([1, 3, 4, 5]);
    /// assert_eq!(
    ///     edge.ranges().collect::<Vec<_>>(),
    ///     [RangeU8::new(1, 1), RangeU8::new(3, 5)],
    /// );
    /// ```
    pub fn ranges(&self) -> RangeIter {
        self.0.symbols.ranges()
    }

    /// Returns an iterator over all tags in this edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_graph::{Graph, Tag};
    ///
    /// let gr = Graph::new();
    /// let edge = gr.node().connect(gr.node());
    /// edge.add_tag(Tag::OpenGroup(1));
    /// edge.add_tag(Tag::CloseGroup(1));
    /// assert_eq!(
    ///     edge.tags().collect::<Vec<_>>(),
    ///     [Tag::OpenGroup(1), Tag::CloseGroup(1)],
    /// );
    /// ```
    #[inline]
    pub fn tags(&self) -> impl Iterator<Item = Tag> {
        self.0.tags.iter().copied()
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
