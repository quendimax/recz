use crate::edge::{Edge, EdgePtr};
use crate::graph::{GraphInner, GraphPtr};
use recz_adt::Map;
use std::cell::Cell;
use std::fmt::Write;
use std::iter::Iterator;

/// Node for an NFA graph.
///
/// It contains ID (unique within its graph owner). Also it can be connected to
/// another node via [`Transition`]'s.
pub struct Node<'a>(&'a NodeInner);

pub(crate) struct NodeInner {
    uid: u64,
    is_final: Cell<bool>,
    sources: Map<NodePtr, EdgePtr>,
    targets: Map<NodePtr, EdgePtr>,
    graph_ptr: GraphPtr,
}

pub(crate) type NodePtr = core::ptr::NonNull<NodeInner>;

/// Public API
impl<'a> Node<'a> {
    /// Returns the node's identifier that is unique within its owner.
    #[inline]
    pub fn nid(&self) -> u32 {
        (self.0.uid & Self::ID_MASK) as u32
    }

    /// Returns the node's graph owner identifier.
    #[inline]
    pub fn gid(&self) -> u32 {
        (self.0.uid >> Self::ID_BITS) as u32
    }

    /// Returns the node's identifier unique within the running process.
    #[inline]
    pub fn uid(&self) -> u64 {
        self.0.uid
    }

    /// Checks if the node is a final N/DFA state.
    #[inline]
    pub fn is_final(&self) -> bool {
        self.0.is_final.get()
    }

    /// Make the node final.
    pub fn finalize(&self) -> Self {
        self.0.is_final.set(true);
        *self
    }

    /// Make the node non-final.
    pub fn definalize(&self) -> Self {
        self.0.is_final.set(false);
        *self
    }

    /// Creates a new empty edge between two nodes. You can fill the edge with
    /// symbols and tags later.
    pub fn connect(&self, to: Node<'a>) -> Edge<'a> {
        assert_eq!(
            self.gid(),
            to.gid(),
            "only nodes belonging to the same graph can be joined"
        );
        if let Some(edge) = self.0.targets.get(&to.as_ptr()) {
            Edge::from_ref(unsafe { edge.as_ref() })
        } else {
            let edge = self.0.graph_inner().edge();
            self.0.targets.insert(to.as_ptr(), edge.as_ptr());
            to.0.sources.insert(self.as_ptr(), edge.as_ptr());
            edge
        }
    }

    /// Returns an iterator over target nodes, i.e. nodes that this node has
    /// transitions to.
    ///
    /// This iterator walks over pairs `(Edge<'a>, Node<'a>)`.
    #[inline]
    pub fn targets(&self) -> impl Iterator<Item = (Edge<'a>, Node<'a>)> {
        self.0.targets.iter().map(|(to, tr)| {
            let node = Node::from_ref(unsafe { to.as_ref() });
            let edge = Edge::from_ref(unsafe { tr.as_ref() });
            (edge, node)
        })
    }

    /// Returns an iterator over target nodes, i.e. nodes that this node has
    /// transitions from.
    ///
    /// This iterator walks over pairs `(Node<'a>, Edge<'a>)`.
    #[inline]
    pub fn sources(&self) -> impl Iterator<Item = (Node<'a>, Edge<'a>)> {
        self.0.sources.iter().map(|(to, edge)| {
            let node = Node::from_ref(unsafe { to.as_ref() });
            let edge = Edge::from_ref(unsafe { edge.as_ref() });
            (node, edge)
        })
    }
}

/// Private API
impl<'a> Node<'a> {
    pub(crate) const ID_MASK: u64 = (1 << (u64::BITS / 2)) - 1;
    pub(crate) const ID_BITS: u32 = u64::BITS / 2;

    pub(crate) fn from_ref(inner: &'a NodeInner) -> Self {
        Self(inner)
    }

    pub(crate) fn as_ptr(&self) -> NodePtr {
        NodePtr::from(self.0)
    }
}

/// Crate API
impl NodeInner {
    #[inline(always)]
    pub(crate) fn new(graph: &GraphInner, gid: u32, nid: u32) -> NodeInner {
        let uid = ((gid as u64) << Node::ID_BITS) | nid as u64;
        NodeInner {
            uid,
            is_final: Cell::new(false),
            sources: Default::default(),
            targets: Default::default(),
            graph_ptr: GraphPtr::from(graph),
        }
    }

    /// Arena owner of this node.
    #[inline]
    fn graph_inner<'a>(&self) -> &'a GraphInner {
        unsafe { self.graph_ptr.as_ref() }
    }
}

impl Copy for Node<'_> {}

impl Clone for Node<'_> {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::cmp::Eq for Node<'_> {}

impl std::cmp::PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.uid().eq(&other.uid())
    }
}

impl std::cmp::Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uid().cmp(&other.uid())
    }
}

impl std::cmp::PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Node<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uid().hash(state)
    }
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl std::fmt::$trait for Node<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.is_final() {
                    f.write_str("node((")?;
                } else {
                    f.write_str("node(")?;
                }
                std::fmt::$trait::fmt(&self.nid(), f)?;
                if self.is_final() {
                    f.write_str("))")
                } else {
                    f.write_char(')')
                }
            }
        }
    };
}

impl_fmt!(std::fmt::Display);
impl_fmt!(std::fmt::Debug);
impl_fmt!(std::fmt::Binary);
impl_fmt!(std::fmt::Octal);
impl_fmt!(std::fmt::UpperHex);
impl_fmt!(std::fmt::LowerHex);
