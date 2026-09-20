use crate::{Edge, Node};
use recz_adt::{DefaultHasher, Map, Set};
use std::hash::{Hash, Hasher};

pub(super) struct PathFinder<'a, F: FnMut(&[Edge<'a>])> {
    start_node: Node<'a>,
    end_node: Node<'a>,
    handler: F,

    path: Vec<Edge<'a>>,
    visited: Map<Node<'a>, Set<u64>>,
    hashers: Vec<DefaultHasher>,
}

const INIT_BLOB: u64 = 0xDEADBEEF;

impl<'a, F: FnMut(&[Edge<'a>])> PathFinder<'a, F> {
    pub(super) fn new(start_node: Node<'a>, end_node: Node<'a>, handler: F) -> Self {
        let mut hashers = Vec::default();
        let mut init_hasher = DefaultHasher::default();
        init_hasher.write_u64(INIT_BLOB);
        hashers.push(init_hasher);
        Self {
            start_node,
            end_node,
            handler,
            path: Vec::default(),
            visited: Map::default(),
            hashers,
        }
    }

    pub(super) fn run(&mut self) {
        self.path.clear();
        self.visited.clear();

        assert_eq!(self.hashers.len(), 1);
        self.recurse(self.start_node);
        assert_eq!(self.hashers.len(), 1);
    }

    fn recurse(&mut self, node: Node<'a>) {
        let new_check = self.hashers.last().unwrap().finish();
        let passed_checks = self.visited.entry(node).or_default();
        if passed_checks.contains(&new_check) {
            return;
        }
        passed_checks.insert(new_check);

        if node == self.end_node {
            (self.handler)(&self.path);
        }

        for (edge, target) in node.targets() {
            self.push(edge);
            self.recurse(target);
            self.pop(edge);
        }
    }

    fn push(&mut self, edge: Edge<'a>) {
        if !self.path.contains(&edge) {
            let mut hasher = self.hashers.last().unwrap().clone();
            edge.hash(&mut hasher);
            self.hashers.push(hasher);
        }
        self.path.push(edge);
    }

    fn pop(&mut self, edge: Edge<'a>) {
        assert_eq!(self.path.pop(), Some(edge));
        if !self.path.contains(&edge) {
            self.hashers.pop();
        }
    }
}
