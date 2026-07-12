use crate::graph::Graph;
use crate::transition::{TransPtr, Transition};
use redt::{Map, MapIter};
use smallvec::{SmallVec, smallvec};
use std::cell::{Cell, Ref, RefCell};
use std::fmt::Write;
use std::ops::Deref;
use std::ptr::NonNull;

/// Node for an NFA graph.
///
/// It contains ID (unique within its graph owner). Also it can be connected to
/// another node via [`Transition`]'s.
pub struct Node<'a>(&'a NodeInner);

pub(crate) type NodePtr = NonNull<NodeInner>;
pub(crate) type GraphPtr = NonNull<Graph>;

// most node pairs have only one transition
type TransVec = SmallVec<[TransPtr; 1]>;

pub(crate) struct NodeInner {
    uid: u64,
    is_final: Cell<bool>,
    sources: RefCell<Map<NodePtr, TransVec>>,
    targets: RefCell<Map<NodePtr, TransVec>>,
    graph: GraphPtr,
}

/// Public API
impl<'a> Node<'a> {
    pub(crate) const ID_MASK: u64 = (1 << (u64::BITS / 2)) - 1;
    pub(crate) const ID_BITS: u32 = u64::BITS / 2;

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

    /// Arena owner of this node.
    #[inline]
    pub fn graph(&self) -> &'a Graph {
        unsafe { self.0.graph.as_ref() }
    }

    /// Creates a new empty transition between two nodes. You can fill the
    /// transition with symbols later.
    ///
    /// Specifying instruction is mandatory. If there is no instruction for the
    /// transition use [`nop`].
    ///
    /// [`nop`]: crate::isa::Inst::Nop
    pub fn connect(&self, to: Node<'a>) -> Transition<'a> {
        assert_eq!(
            self.gid(),
            to.gid(),
            "only nodes belonging to the same graph can be joined"
        );
        let mut self_targets = self.0.targets.borrow_mut();
        let mut to_sources = to.0.sources.borrow_mut();
        if let Some(self_tr_vec) = self_targets.get_mut(&to.as_ptr()) {
            let to_tr_vec = to_sources.get_mut(&self.as_ptr()).unwrap();
            let tr = self.graph().transition();
            self_tr_vec.push(tr.as_ptr());
            to_tr_vec.push(tr.as_ptr());
            tr
        } else {
            let tr = self.graph().transition();
            self_targets.insert(to.as_ptr(), smallvec![tr.as_ptr()]);
            to_sources.insert(self.as_ptr(), smallvec![tr.as_ptr()]);
            tr
        }
    }

    /// Returns an iterator over target nodes, i.e. nodes that this node is
    /// connected to.
    ///
    /// This iterator walks over pairs ([`Node`], [`Transition`]). Because of
    /// `Transition` contains only one instruction, it's possible to get the
    /// same node multiple times.
    #[inline]
    pub fn targets(&self) -> TargetNodeIter<'a> {
        let lock = self.0.targets.borrow();
        TargetNodeIter::new(lock)
    }
}

/// Crate API
impl<'a> Node<'a> {
    #[inline(always)]
    pub(crate) fn new_inner(graph: &'a Graph, gid: u32, nid: u32) -> NodeInner {
        let uid = ((gid as u64) << Node::ID_BITS) | nid as u64;
        NodeInner {
            uid,
            is_final: Cell::new(false),
            sources: Default::default(),
            targets: Default::default(),
            graph: NonNull::from(graph),
        }
    }

    pub(crate) fn as_ptr(&self) -> NodePtr {
        NodePtr::from(self.0)
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

impl<'a> std::convert::From<&'a NodeInner> for Node<'a> {
    fn from(inner: &'a NodeInner) -> Self {
        Self(inner)
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

/// Iterator over the targets of a node.
///
/// Use it as iterator only by reference.
pub struct TargetNodeIter<'a> {
    // lock guarantees that the map is not modified while iterating
    #[allow(unused)]
    lock: Ref<'a, Map<NodePtr, TransVec>>,
    iter: MapIter<'a, NodePtr, TransVec>,
}

impl<'a> TargetNodeIter<'a> {
    fn new(map: Ref<'a, Map<NodePtr, TransVec>>) -> Self {
        unsafe {
            let map_ptr = map.deref() as *const Map<NodePtr, TransVec>;
            let iter = (*map_ptr).iter();
            Self { lock: map, iter }
        }
    }

    /// Iterator over the nodes of the targets of a node.
    pub fn nodes(self) -> impl Iterator<Item = Node<'a>> {
        self.map(|(node, _)| node)
    }
}

impl<'a> Iterator for TargetNodeIter<'a> {
    type Item = (Node<'a>, Box<[Transition<'a>]>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(target, tr_vec)| {
            let node = Node::from(unsafe { target.as_ref() });
            let tr = tr_vec
                .iter()
                .map(|tr| Transition::from(unsafe { tr.as_ref() }))
                .collect();
            (node, tr)
        })
    }
}
