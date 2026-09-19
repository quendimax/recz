use crate::{Node, Tag};
use recz_adt::{DefaultHasher, Map, Set};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub(super) struct TagCollector {
    tags: Vec<Tag>,
    hashers: Vec<DefaultHasher>,
}

const INIT_BLOB: u64 = 0xDEADBEEF;

impl TagCollector {
    pub(super) fn new() -> Self {
        let mut initial_hasher = DefaultHasher::default();
        initial_hasher.write_u64(INIT_BLOB);
        Self {
            tags: Vec::default(),
            hashers: vec![initial_hasher],
        }
    }

    pub(super) fn clear(&mut self) {
        self.tags.clear();
        self.hashers.clear();
        let mut initial_hasher = DefaultHasher::default();
        initial_hasher.write_u64(INIT_BLOB);
        self.hashers.push(initial_hasher);
    }

    pub(super) fn insert(&mut self, tag: Tag) {
        if !self.tags.contains(&tag) {
            let mut hasher = self.hashers.last().unwrap().clone();
            tag.hash(&mut hasher);
            self.hashers.push(hasher);
        }
        self.tags.push(tag);
    }

    pub(super) fn extend(&mut self, tags: impl IntoIterator<Item = Tag>) {
        for tag in tags.into_iter() {
            self.insert(tag);
        }
    }

    pub(super) fn remove(&mut self, tag: Tag) {
        assert_eq!(self.tags.pop(), Some(tag));
        if !self.tags.contains(&tag) {
            self.hashers.pop();
        }
    }

    pub(super) fn shorten(&mut self, tags: impl IntoIterator<Item = Tag>) {
        for tag in tags.into_iter() {
            self.remove(tag);
        }
    }

    pub(super) fn tags(&self) -> impl Iterator<Item = Tag> + '_ {
        self.tags.iter().copied()
    }

    pub(super) fn checksum(&self) -> u64 {
        self.hashers.last().unwrap().finish()
    }
}

pub(super) struct PathFinder<'a, F: FnMut(&[Node<'a>])> {
    start_node: Node<'a>,
    tag_collector: TagCollector,
    visited: Map<Node<'a>, Set<u64>>,
    path: Vec<Node<'a>>,
    handler: F,
}

impl<'a, F: FnMut(&[Node<'a>])> PathFinder<'a, F> {
    pub(super) fn new(start_node: Node<'a>, handler: F) -> Self {
        Self {
            start_node,
            tag_collector: TagCollector::new(),
            visited: Map::default(),
            path: Vec::default(),
            handler,
        }
    }

    pub(super) fn run(&mut self) {
        self.tag_collector.clear();
        self.visited.clear();
        self.recurse(self.start_node);
    }

    fn recurse(&mut self, node: Node<'a>) {
        let new_check = self.tag_collector.checksum();
        let passed_checks = self.visited.entry(node).or_default();
        if passed_checks.contains(&new_check) {
            return;
        }
        if node.is_epilogue() {
            self.path.push(node);
            (self.handler)(&self.path);
            self.path.pop();
            return;
        }
        passed_checks.insert(new_check);
        self.path.push(node);

        for (edge, target) in node.targets() {
            self.tag_collector.extend(edge.tags());
            self.recurse(target);
            self.tag_collector.shorten(edge.tags().rev());
        }

        self.path.pop();
    }
}
