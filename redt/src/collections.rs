//! LinkedHashMap/Set preserves insertion order of keys. It needs only to get
//! better legibility of printed graphs. Also it will allow to run some
//! benchmarks with different algorithms inside the sets and maps.

use fnv::FnvBuildHasher;
use ordermap::{OrderMap, OrderSet};

pub type Map<K, V> = OrderMap<K, V, FnvBuildHasher>;
pub type Set<T> = OrderSet<T, FnvBuildHasher>;
