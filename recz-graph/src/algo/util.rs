use crate::Tag;
use recz_adt::DefaultHasher;
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

    pub(super) fn contains(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub(super) fn tags(&self) -> impl Iterator<Item = Tag> + '_ {
        self.tags.iter().copied()
    }

    pub(super) fn checksum(&self) -> u64 {
        self.hashers.last().unwrap().finish()
    }
}
