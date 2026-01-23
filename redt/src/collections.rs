use fnv::FnvBuildHasher;
use ordermap::{OrderMap, OrderSet, map, set};

pub type Map<K, V> = OrderMap<K, V, FnvBuildHasher>;
pub type MapIter<'a, K, V> = map::Iter<'a, K, V>;

pub type Set<T> = OrderSet<T, FnvBuildHasher>;
pub type SetIter<'a, T> = set::Iter<'a, T>;

pub use bump_stack::Stack;
pub use smallvec::{SmallVec, smallvec};
