//! LinkedHashMap/Set preserves insertion order of keys. It needs only to get
//! better legibility of printed graphs. Also it will allow to run some
//! benchmarks with different algorithms inside the sets and maps.

use fnv::FnvBuildHasher;
use ordermap::{OrderMap, OrderSet, map, set};

pub type Map<K, V> = OrderMap<K, V, FnvBuildHasher>;
pub type MapIter<'a, K, V> = map::Iter<'a, K, V>;

pub type Set<T> = OrderSet<T, FnvBuildHasher>;
pub type SetIter<'a, T> = set::Iter<'a, T>;
