use crate::node::{Node, NodeInner, NodePtr};
use crate::tag::{Inst, Tag};
use crate::transition::{TransInner, Transition};
use redt::{Set, Stack};
use std::cell::Cell;
use std::fmt::Write;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Graph {
    gid: u32,
    next_nid: Cell<u32>,
    start_node: Cell<Option<NodePtr>>,
    bump_nodes: Stack<NodeInner>,
    bump_trans: Stack<TransInner>,
    tag_id: Cell<usize>,
}

static NEXT_GRAPH_ID: AtomicU32 = AtomicU32::new(1);

impl Graph {
    pub fn new() -> Self {
        let gid = NEXT_GRAPH_ID.fetch_add(1, Ordering::Relaxed);
        if gid == 0 {
            panic!("graph id overflow");
        }

        Self {
            gid,
            next_nid: Cell::new(0),
            start_node: Cell::new(None),
            bump_nodes: Stack::new(),
            bump_trans: Stack::new(),
            tag_id: Cell::new(0),
        }
    }

    /// This graph's ID.
    #[inline]
    pub fn gid(&self) -> u32 {
        self.gid
    }

    /// Creates a new node.
    pub fn node(&self) -> Node<'_> {
        let nid = self.next_nid.replace(
            self.next_nid
                .get()
                .checked_add(1)
                .expect("node id overflow"),
        );
        let node_ref = self
            .bump_nodes
            .push_with(|| Node::new_inner(self, self.gid, nid));

        let node_ptr = NodePtr::from(node_ref);

        if self.start_node.get().is_none() {
            self.start_node.set(Some(node_ptr));
        }
        Node::from(node_ref)
    }

    /// Creates a new transition.
    pub(crate) fn transition(&self, with: Inst) -> Transition<'_> {
        let tr_ref = self.bump_trans.push_with(|| Transition::new_inner(with));
        Transition::from(tr_ref)
    }

    pub fn tag(&self) -> Tag {
        let id = self.tag_id.get();
        self.tag_id.set(id + 1);
        Tag::new(id)
    }

    /// Returns the start node of the graph. If the graph is empty, creates a
    /// node, and returns it.
    #[inline]
    pub fn start_node(&self) -> Node<'_> {
        if let Some(ptr) = self.start_node.get() {
            Node::from(unsafe { ptr.as_ref() })
        } else {
            self.node()
        }
    }

    /// Returns true if the graph is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start_node.get().is_none()
    }

    /// Returns a number of nodes belonging to this graph.
    #[inline]
    pub fn len(&self) -> usize {
        self.bump_nodes.len()
    }
}

impl std::default::Default for Graph {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl ::std::fmt::$trait for Graph {
            #[allow(clippy::mutable_key_type)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fn recurse<'a>(node: Node<'a>, visited: &mut Set<Node<'a>>) {
                    visited.insert(node);
                    for (target, _) in node.targets() {
                        if !visited.contains(&target) {
                            recurse(target, visited);
                        }
                    }
                }
                if self.start_node.get().is_none() {
                    return Ok(());
                }
                let start_node = self.start_node();
                let mut visited = Set::default();
                recurse(start_node, &mut visited);
                let mut first = true;
                for node in visited.iter().copied() {
                    if first {
                        first = false;
                    } else {
                        f.write_char('\n')?;
                    }
                    let mut is_empty = true;
                    ::std::fmt::$trait::fmt(&node, f)?;
                    f.write_str(" {")?;
                    for (target, transitions) in node.targets() {
                        for transition in transitions {
                            f.write_str("\n    ")?;
                            ::std::fmt::$trait::fmt(&transition, f)?;
                            f.write_str(" -> ")?;
                            if node == target {
                                f.write_str("self")?;
                            } else {
                                ::std::fmt::$trait::fmt(&target, f)?;
                            }
                            is_empty = false;
                        }
                    }
                    if !is_empty {
                        f.write_char('\n')?;
                    }
                    f.write_char('}')?;
                }
                Ok(())
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
