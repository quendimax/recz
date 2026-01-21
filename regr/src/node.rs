use crate::arena::Arena;
use crate::isa::Inst;
use crate::transition::Transition;
use redt::{Map, MapIter};
use smallvec::smallvec;
use std::cell::{Cell, Ref, RefCell};
use std::fmt::Write;
use std::ops::Deref;

/// Node for an NFA graph.
///
/// It contains ID (unique within its graph owner). Also it can be connected to
/// another node via [`Transition`]'s.
pub struct Node<'a>(&'a NodeInner<'a>);

type TrVec<'a> = smallvec::SmallVec<[Transition<'a>; 1]>;

pub(crate) struct NodeInner<'a> {
    uid: u64,
    is_final: Cell<bool>,
    targets: RefCell<Map<Node<'a>, TrVec<'a>>>,
    arena: &'a Arena,
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
    pub fn arena(&self) -> &'a Arena {
        self.0.arena
    }

    /// Creates a new empty transition between two nodes. You can fill the
    /// transition with symbols later.
    ///
    /// Specifying instruction is mandatory. If there is no instruction for the
    /// transition use [`nop`].
    ///
    /// [`nop`]: crate::isa::Inst::Nop
    pub fn connect(&self, to: Node<'a>, with: Inst) -> Transition<'a> {
        assert_eq!(
            self.gid(),
            to.gid(),
            "only nodes of the same graph can be joint"
        );
        let mut targets = self.0.targets.borrow_mut();
        if let Some(tr_vec) = targets.get_mut(&to) {
            if let Some(tr) = tr_vec.iter().find(|tr| tr.instruct() == with) {
                *tr
            } else {
                let tr = Transition::new(*self, to, with);
                tr_vec.push(tr);
                tr
            }
        } else {
            let tr = Transition::new(*self, to, with);
            targets.insert(to, smallvec![tr]);
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
    pub(crate) fn new_in(arena: &'a Arena, gid: u32, nid: u32) -> Node<'a> {
        let uid = ((gid as u64) << Node::ID_BITS) | nid as u64;
        arena.alloc_node_with(|| NodeInner {
            uid,
            is_final: Cell::new(false),
            targets: Default::default(),
            arena,
        })
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

impl<'a> std::convert::From<&'a NodeInner<'a>> for Node<'a> {
    fn from(inner: &'a NodeInner<'a>) -> Self {
        Self(inner)
    }
}

impl<'a> std::convert::From<&'a mut NodeInner<'a>> for Node<'a> {
    fn from(inner: &'a mut NodeInner<'a>) -> Self {
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
    lock: Ref<'a, Map<Node<'a>, TrVec<'a>>>,
    iter: Cell<MapIter<'a, Node<'a>, TrVec<'a>>>,
}

impl<'a> TargetNodeIter<'a> {
    fn new(map: Ref<'a, Map<Node<'a>, TrVec<'a>>>) -> Self {
        unsafe {
            let map_ptr = map.deref() as *const Map<Node<'a>, TrVec<'a>>;
            let iter = (*map_ptr).iter();
            Self {
                lock: map,
                iter: Cell::new(iter),
            }
        }
    }

    /// Iterator over the nodes of the targets of a node.
    pub fn nodes(self) -> impl Iterator<Item = Node<'a>> {
        self.map(|(node, _)| node)
    }
}

impl<'a> Iterator for TargetNodeIter<'a> {
    type Item = (Node<'a>, TrVec<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.iter.take();
        let res = iter
            .next()
            .map(|(target, tr_vec)| (*target, tr_vec.clone()));
        self.iter.set(iter);
        res
    }
}
