use bumpish::{BumpMap, BumpSet, map, set};
use bumpish::{BumpOrdMap, BumpOrdSet, ordmap, ordset};

pub type Map<K, V> = BumpMap<K, V, ahash::RandomState>;
pub type MapIter<'a, K, V> = map::Iter<'a, K, V>;

pub type Set<T> = BumpSet<T, ahash::RandomState>;
pub type SetIter<'a, T> = set::Iter<'a, T>;

pub type OrdMap<K, V> = BumpOrdMap<K, V>;
pub type OrdMapIter<'a, K, V> = ordmap::Iter<'a, K, V>;

pub type OrdSet<T> = BumpOrdSet<T>;
pub type OrdSetIter<'a, T> = ordset::Iter<'a, T>;
