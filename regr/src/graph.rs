use crate::arena::Arena;
use crate::node::Node;
use crate::tag::Tag;
use redt::{Map, Set};
use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Graph<'a> {
    gid: u32,
    arena: &'a Arena,
    next_nid: Cell<u32>,
    start_node: Cell<Option<Node<'a>>>,
    tag_bank: RefCell<Map<u32, Tag>>,          // id -> tag
    tag_groups: RefCell<Map<u32, (u32, u32)>>, // label -> (open_tag_id, close_tag_id)
}

static NEXT_GRAPH_ID: AtomicU32 = AtomicU32::new(1);

impl<'a> Graph<'a> {
    pub fn new_in(arena: &'a mut Arena) -> Self {
        let gid = NEXT_GRAPH_ID.fetch_add(1, Ordering::Relaxed);
        if gid == 0 {
            panic!("graph id overflow");
        }

        arena.bind_graph(gid);

        Self {
            gid,
            arena,
            next_nid: Cell::new(0),
            start_node: Cell::new(None),
            tag_bank: RefCell::new(Map::default()),
            tag_groups: RefCell::new(Map::default()),
        }
    }

    /// This graph's ID.
    #[inline]
    pub fn gid(&self) -> u32 {
        self.gid
    }

    /// Creates a new node.
    pub fn node(&self) -> Node<'a> {
        let nid = self.next_nid.replace(
            self.next_nid
                .get()
                .checked_add(1)
                .expect("node id overflow"),
        );
        let node = Node::new_in(self.arena, self.gid, nid);
        if self.start_node.get().is_none() {
            self.start_node.set(Some(node));
        }
        node
    }

    /// Returns the start node of the graph. If the graph is empty, creates a
    /// node, and returns it.
    #[inline]
    pub fn start_node(&self) -> Node<'a> {
        self.start_node.get().unwrap_or_else(|| self.node())
    }

    /// Returns true if the graph is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start_node.get().is_none()
    }

    /// Arena owner of the graph's nodes and transitions.
    #[inline]
    pub fn arena(&self) -> &'a Arena {
        self.arena
    }

    pub fn add_tag_group(&self, label: u32, open_tag: Tag, close_tag: Tag) {
        let open_id = open_tag.id();
        let close_id = close_tag.id();

        let mut tag_bank = self.tag_bank.borrow_mut();
        assert_eq!(tag_bank.entry(open_id).or_insert(open_tag).id(), open_id);
        assert_eq!(tag_bank.entry(close_id).or_insert(close_tag).id(), close_id);

        let mut tag_table = self.tag_groups.borrow_mut();
        tag_table.entry(label).or_insert((open_id, close_id));
    }

    pub fn tag_group(&self, label: u32) -> Option<(Tag, Tag)> {
        if let Some((open_id, close_id)) = self.tag_groups.borrow().get(&label).cloned() {
            let tag_bank = self.tag_bank.borrow();
            Some((tag_bank[&open_id], tag_bank[&close_id]))
        } else {
            None
        }
    }

    /// Returns an iterator over all tag groups in the graph. The iterator
    /// yields tuples of the form `(label, (open_tag, close_tag))`.
    pub fn tag_groups(&self) -> impl std::iter::Iterator<Item = (u32, (Tag, Tag))> {
        TagGroupIter::new(self)
    }

    /// Visits each node of the graph, i.e. every node reachable from the start
    /// node.
    pub fn for_each_node<F>(&self, f: F)
    where
        F: FnMut(Node<'a>),
    {
        struct Lambda<'a, F: FnMut(Node<'a>)> {
            visited: Set<Node<'a>>,
            handler: F,
        }
        impl<'a, F: FnMut(Node<'a>)> Lambda<'a, F> {
            fn visit(&mut self, node: Node<'a>) {
                self.visited.insert(node);
                (self.handler)(node);
                for target in node.targets().keys() {
                    if !self.visited.contains(target) {
                        self.visit(*target);
                    }
                }
            }
        }
        Lambda {
            visited: Set::default(),
            handler: f,
        }
        .visit(self.start_node());
    }
}

impl std::ops::Drop for Graph<'_> {
    fn drop(&mut self) {
        self.arena.unbind_graph();
    }
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl ::std::fmt::$trait for Graph<'_> {
            #[allow(clippy::mutable_key_type)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fn recurse<'a>(node: Node<'a>, visited: &mut BTreeSet<Node<'a>>) {
                    visited.insert(node);
                    for target in node.targets().keys().copied() {
                        if !visited.contains(&target) {
                            recurse(target, visited);
                        }
                    }
                }
                if let Some(start_node) = self.start_node.get() {
                    let mut visited = BTreeSet::new();
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
                        for (target, transition) in node.targets().iter() {
                            f.write_str("\n    ")?;
                            ::std::fmt::$trait::fmt(transition, f)?;
                            f.write_str(" -> ")?;
                            if node == *target {
                                f.write_str("self")?;
                            } else {
                                ::std::fmt::$trait::fmt(&target, f)?;
                            }
                            is_empty = false;
                        }
                        if !is_empty {
                            f.write_char('\n')?;
                        }
                        f.write_char('}')?;
                    }
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

struct TagGroupIter<'a, 'g> {
    graph: &'g Graph<'a>,
    labels: Vec<u32>,
    index: usize,
}

impl<'a, 'g> TagGroupIter<'a, 'g> {
    pub fn new(graph: &'g Graph<'a>) -> Self {
        Self {
            graph,
            labels: graph
                .tag_groups
                .borrow()
                .keys()
                .cloned()
                .collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl<'a, 'g> Iterator for TagGroupIter<'a, 'g> {
    type Item = (u32, (Tag, Tag));

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.labels.len() {
            let label = self.labels[self.index];
            self.index += 1;
            Some((label, self.graph.tag_group(label).unwrap()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod utest {
    use super::*;

    #[test]
    #[should_panic(expected = "graph id overflow")]
    fn graph_ctor_panic() {
        NEXT_GRAPH_ID.store(u32::MAX, Ordering::Relaxed);
        let mut arena = Arena::new();
        _ = Graph::new_in(&mut arena);
        _ = Graph::new_in(&mut arena);
    }
}
