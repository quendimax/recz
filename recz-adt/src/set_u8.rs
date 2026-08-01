use crate::{Legible, RangeU8};
use owo_colors::OwoColorize;
use std::cell::Cell;
use std::fmt::Write;

type Chunk = usize;

/// Number of bits that are needed to represent how many separate bits the chunk
/// can hold. E.g. for 64-bit chunks, this is 6.
const LOG2_CHUNK_BITS: u32 = Chunk::BITS.trailing_zeros();

/// Quantity of `Chunk` values in the `chunks` member for symbols' bits.
const BITMAP_LEN: usize = (u8::MAX as usize + 1) / Chunk::BITS as usize;

/// Width of the bitmap in bits.
const BITMAP_WIDTH: u32 = {
    let width = Chunk::BITS * BITMAP_LEN as u32;
    assert!(width == 256);
    width
};

/// Returns the index of the chunk that contains the given byte.
const fn chunk_index(byte: u8) -> usize {
    byte as usize >> LOG2_CHUNK_BITS
}

/// Returns the mask for the given byte within its chunk.
const fn chunk_mask(byte: u8) -> Chunk {
    1 << (byte & (u8::MAX >> (u8::BITS - LOG2_CHUNK_BITS)))
}

/// A set of symbols that can be used to represent any byte.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetU8 {
    bitmap: [Cell<Chunk>; BITMAP_LEN],
}

impl SetU8 {
    /// The number of chunks in the inner bitmap.
    pub const CHUNKS: usize = BITMAP_LEN;

    /// Creates a new empty byte set.
    ///
    /// Because of the set uses a fixed-size bitmap under the hood, the capacity
    /// is always 256.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    /// let set = SetU8::new();
    /// assert_eq!(set.len(), 0);
    /// assert_eq!(set.capacity(), 256);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            bitmap: [const { Cell::new(0) }; BITMAP_LEN],
        }
    }

    /// Creates a new byte set that contains all possible bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    /// let set = SetU8::full();
    /// assert_eq!(set.len(), 256);
    /// assert_eq!(set.capacity(), 256);
    #[inline]
    pub fn full() -> Self {
        Self {
            bitmap: [const { Cell::new(Chunk::MAX) }; BITMAP_LEN],
        }
    }

    /// Returns the number of bytes in the set.
    pub fn len(&self) -> usize {
        self.bitmap
            .iter()
            .fold(0, |acc, ch| acc + ch.get().count_ones()) as usize
    }

    /// Checks if the set is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bitmap.iter().all(|chunk| chunk.get() == 0)
    }

    /// Returns the capacity of the set, which is always 256, for every possible
    /// byte value, because the set uses a fixed-size bitmap under the hood.
    pub const fn capacity(&self) -> usize {
        BITMAP_WIDTH as usize
    }

    /// Clears the set, removing all bytes.
    pub fn clear(&self) {
        self.bitmap.iter().for_each(|ch| ch.set(0));
    }

    /// Inserts a byte into the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    /// - If the set did not previously contain the byte, `true` is returned.
    /// - If the set already contained the byte, `false` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_adt::SetU8;
    /// let set = SetU8::new();
    ///
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn insert(&self, byte: u8) -> bool {
        let chunk_index = chunk_index(byte);
        let chunk_mask = chunk_mask(byte);
        let old_chunk = self.bitmap[chunk_index].get();
        self.bitmap[chunk_index].set(old_chunk | chunk_mask);
        old_chunk & chunk_mask == 0
    }

    /// Inserts a byte into the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_adt::SetU8;
    /// let set = SetU8::new();
    ///
    /// set.insert(2);
    /// set.insert(4);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn insert_byte(&self, byte: u8) {
        self.bitmap[chunk_index(byte)].update(|chunk| chunk | chunk_mask(byte));
    }

    /// Inserts bytes into the set. The bytes are taken from the value that
    /// implements [`Into<SetU8>`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// # use recz_adt::SetU8;
    /// let set = SetU8::new();
    ///
    /// set.insert_bytes([2, 4]);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn insert_bytes(&self, bytes: impl Into<SetU8>) {
        let bytes = bytes.into();
        for i in 0..BITMAP_LEN {
            self.bitmap[i].update(|ch| ch | bytes.bitmap[i].get());
        }
    }

    /// If the set contains a specified byte, removes it from the set. Returns
    /// whether such a byte was present.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::new();
    ///
    /// set.insert(2);
    /// assert_eq!(set.remove(2), true);
    /// assert_eq!(set.remove(2), false);
    /// ```
    pub fn remove(&self, value: u8) -> bool {
        let byte = value;
        let chunk_index = chunk_index(byte);
        let chunk_mask = chunk_mask(byte);
        let old_chunk = self.bitmap[chunk_index].get();
        self.bitmap[chunk_index].set(old_chunk & !chunk_mask);
        old_chunk & chunk_mask != 0
    }

    /// Removes all specified bytes from the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::from([1, 2, 3, 4]);
    /// set.remove_bytes([2, 3]);
    /// let v = set.iter().collect::<Vec<_>>();
    /// assert_eq!(v, [1, 4]);
    /// ```
    pub fn remove_bytes(&self, bytes: impl Into<SetU8>) {
        let bytes = bytes.into();
        for i in 0..BITMAP_LEN {
            self.bitmap[i].update(|ch| ch & !bytes.bitmap[i].get());
        }
    }

    /// Removes and returns the element in the set, if any, that is equal to
    /// the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::from([1, 2, 3]);
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), None);
    /// ```
    pub fn take(&self, value: &u8) -> Option<u8> {
        let byte = *value;
        let chunk_index = chunk_index(byte);
        let chunk_mask = chunk_mask(byte);
        let old_chunk = self.bitmap[chunk_index].get();
        if old_chunk & chunk_mask != 0 {
            self.bitmap[chunk_index].set(old_chunk & !chunk_mask);
            Some(byte)
        } else {
            None
        }
    }

    /// Returns the first element in the set, if any. This element is always the
    /// minimum of all elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::new();
    /// assert_eq!(set.first(), None);
    /// set.insert(1);
    /// assert_eq!(set.first(), Some(1));
    /// set.insert(2);
    /// assert_eq!(set.first(), Some(1));
    /// ```
    pub fn first(&self) -> Option<u8> {
        let mut chunk = self.bitmap[0].get();
        let mut shift: u32 = 0;
        while shift < BITMAP_WIDTH {
            if chunk != 0 {
                let trailing_zeros = chunk.trailing_zeros();
                let symbol = trailing_zeros + shift;
                return Some(symbol as u8);
            }
            if shift < BITMAP_WIDTH - Chunk::BITS {
                shift += Chunk::BITS;
                chunk = self.bitmap[chunk_index(shift as u8)].get();
                continue;
            }
            break;
        }
        None
    }

    /// Returns the last byte in the set, if any. This element is always the
    /// minimum of all elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::new();
    /// assert_eq!(set.last(), None);
    /// set.insert(2);
    /// assert_eq!(set.last(), Some(2));
    /// set.insert(1);
    /// assert_eq!(set.last(), Some(2));
    /// ```
    pub fn last(&self) -> Option<u8> {
        let mut shift: u32 = BITMAP_WIDTH - Chunk::BITS;
        let mut chunk = self.bitmap[chunk_index(shift as u8)].get();
        loop {
            if chunk != 0 {
                let leading_zeros = chunk.leading_zeros();
                let byte = Chunk::BITS - 1 - leading_zeros + shift;
                debug_assert!(byte <= u8::MAX as u32 + 1);
                return Some(byte as u8);
            }

            if shift != 0 {
                shift -= Chunk::BITS;
                chunk = self.bitmap[chunk_index(shift as u8)].get();
                continue;
            }
            break;
        }
        None
    }

    /// Returns `true` if the set contains an element equal to the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::from([1, 2, 3]);
    /// assert_eq!(set.contains(1), true);
    /// assert_eq!(set.contains(4), false);
    /// ```
    #[must_use]
    pub fn contains(&self, byte: u8) -> bool {
        self.bitmap[chunk_index(byte)].get() & chunk_mask(byte) != 0
    }

    /// Returns `true` if the set contains all specified bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let set = SetU8::from([1, 2, 3]);
    /// assert_eq!(set.contains_bytes([1, 2]), true);
    /// assert_eq!(set.contains_bytes([1, 2, 5]), false);
    /// ```
    #[must_use]
    pub fn contains_bytes(&self, bytes: impl Into<SetU8>) -> bool {
        let bytes = bytes.into();
        for i in 0..BITMAP_LEN {
            if self.bitmap[i].get() & bytes.bitmap[i].get() != bytes.bitmap[i].get() {
                return false;
            }
        }
        true
    }

    /// Returns `true` if `self` has no elements in common with `other`. This is
    /// equivalent to checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::from([1, 2, 3]);
    /// let b = SetU8::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    #[must_use]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        for i in 0..BITMAP_LEN {
            if self.bitmap[i].get() & other.bitmap[i].get() != 0 {
                return false;
            }
        }
        true
    }

    /// Returns `true` if `self` is a subset of `other`, i.e., all elements in
    /// `self` are also in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let b = SetU8::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// assert_eq!(a.is_subset(&b), false);
    /// b.insert(1);
    /// assert_eq!(a.is_subset(&b), true);
    /// ```
    #[must_use]
    pub fn is_subset(&self, other: &Self) -> bool {
        for i in 0..BITMAP_LEN {
            if self.bitmap[i].get() & !other.bitmap[i].get() != 0 {
                return false;
            }
        }
        true
    }

    /// Returns `true` if `self` is a superset of `other`, i.e., all elements in
    /// `other` are also in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let b = SetU8::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// assert_eq!(a.is_superset(&b), false);
    /// a.insert(3);
    /// assert_eq!(a.is_superset(&b), true);
    /// ```
    #[must_use]
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    /// Creates a new set representing the bytes that are in `self` but not in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let b = SetU8::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let diff: Vec<_> = a.difference(&b).iter().collect();
    /// assert_eq!(diff, [1]);
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let result = Self::new();
        for i in 0..BITMAP_LEN {
            result.bitmap[i].set(self.bitmap[i].get() & !other.bitmap[i].get());
        }
        result
    }

    /// Creates a new set representing the symmetric difference of `self` and
    /// `other`, i.e., the elements that are in `self` or in `other` but not in
    /// both.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let b = SetU8::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let sym_diff: Vec<_> = a.symmetric_difference(&b).iter().collect();
    /// assert_eq!(sym_diff, [1, 3]);
    /// ```
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        let result = Self::new();
        for i in 0..BITMAP_LEN {
            result.bitmap[i].set(self.bitmap[i].get() ^ other.bitmap[i].get());
        }
        result
    }

    /// Creates a new set representing the intersection of `self` and `other`,
    /// i.e., the elements that are both in `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let b = SetU8::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let intersection: Vec<_> = a.intersection(&b).iter().collect();
    /// assert_eq!(intersection, [2]);
    /// ```
    pub fn intersection(&self, other: &Self) -> Self {
        let result = Self::new();
        for i in 0..BITMAP_LEN {
            result.bitmap[i].set(self.bitmap[i].get() & other.bitmap[i].get());
        }
        result
    }

    /// Creates a new set representing the union of `self` and `other`, i.e.,
    /// all the elements in `self` or `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use recz_adt::SetU8;
    ///
    /// let a = SetU8::new();
    /// a.insert(1);
    ///
    /// let b = SetU8::new();
    /// b.insert(2);
    ///
    /// let union: Vec<_> = a.union(&b).iter().collect();
    /// assert_eq!(union, [1, 2]);
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        let result = Self::new();
        for i in 0..BITMAP_LEN {
            result.bitmap[i].set(self.bitmap[i].get() | other.bitmap[i].get());
        }
        result
    }

    /// Returns an iterator over the bytes in the set.
    pub fn iter(&self) -> ByteIter {
        self.bytes()
    }

    /// Returns an iterator over the bytes in the set.
    pub fn bytes(&self) -> ByteIter {
        ByteIter::new(self.clone())
    }

    /// Returns an iterator over the inclusive byte ranges in the set.
    pub fn ranges(&self) -> RangeIter {
        RangeIter::new(self.clone())
    }
}

impl std::default::Default for SetU8 {
    /// Creates a new empty symbol set.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::convert::From<u8> for SetU8 {
    fn from(value: u8) -> Self {
        let set = Self::new();
        set.insert(value);
        set
    }
}

impl std::convert::From<std::ops::RangeInclusive<u8>> for SetU8 {
    fn from(value: std::ops::RangeInclusive<u8>) -> Self {
        Self::from(&RangeU8::from(value))
    }
}

impl std::convert::From<RangeU8> for SetU8 {
    #[inline]
    fn from(value: RangeU8) -> Self {
        Self::from(&value)
    }
}

impl std::convert::From<&RangeU8> for SetU8 {
    fn from(range: &RangeU8) -> Self {
        let mut ls_mask = chunk_mask(range.start());
        ls_mask = !(ls_mask - 1);

        let mut ms_mask = chunk_mask(range.last());
        ms_mask |= ms_mask - 1;

        let ls_index = chunk_index(range.start());
        let ms_index = chunk_index(range.last());

        let result = Self::new();
        if ls_index == ms_index {
            result.bitmap[ms_index].set(ls_mask & ms_mask);
        } else {
            result.bitmap[ls_index].set(ls_mask);
            result.bitmap[ms_index].set(ms_mask);
            for i in 0..ms_index - ls_index - 1 {
                result.bitmap[ls_index + i + 1].set(Chunk::MAX);
            }
        }
        result
    }
}

impl std::convert::From<&[u8]> for SetU8 {
    fn from(value: &[u8]) -> Self {
        let set = Self::default();
        for byte in value {
            set.insert(*byte);
        }
        set
    }
}

impl<const N: usize> std::convert::From<&[u8; N]> for SetU8 {
    fn from(value: &[u8; N]) -> Self {
        Self::from(&value[..])
    }
}

impl<const N: usize> std::convert::From<[u8; N]> for SetU8 {
    fn from(value: [u8; N]) -> Self {
        Self::from(&value[..])
    }
}

impl std::convert::From<&SetU8> for SetU8 {
    fn from(value: &SetU8) -> Self {
        value.clone()
    }
}

impl std::hash::Hash for SetU8 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bitmap.iter().for_each(|chunk| chunk.get().hash(state));
    }
}

macro_rules! impl_ops {
    ($op_assign_trait:ident, $op_assign_fn:ident, $op_trait:ident, $op_fn:ident for $($rhs_ty:ty),+ $(,)?) => {
        $(
            impl ::std::ops::$op_assign_trait<$rhs_ty> for SetU8 {
                #[inline]
                fn $op_assign_fn(&mut self, rhs: $rhs_ty) {
                    use ::std::ops::$op_trait;
                    let rhs = Self::from(rhs);
                    for i in 0..BITMAP_LEN {
                        self.bitmap[i].update(|ch| $op_trait::$op_fn(ch, rhs.bitmap[i].get()));
                    }
                }
            }

            impl ::std::ops::$op_trait<$rhs_ty> for SetU8 {
                type Output = Self;

                #[inline]
                fn $op_fn(self, rhs: $rhs_ty) -> Self {
                    use ::std::ops::$op_trait;
                    let rhs = Self::from(rhs);
                    let new = Self::new();
                    for i in 0..BITMAP_LEN {
                        new.bitmap[i].set($op_trait::$op_fn(self.bitmap[i].get(), rhs.bitmap[i].get()));
                    }
                    new
                }
            }
        )+
    };
}

impl_ops!(BitAndAssign, bitand_assign, BitAnd, bitand for
    u8, RangeU8, &RangeU8, std::ops::RangeInclusive<u8>, SetU8, &SetU8);
impl_ops!(BitOrAssign, bitor_assign, BitOr, bitor for
    u8, RangeU8, &RangeU8, std::ops::RangeInclusive<u8>, SetU8, &SetU8);
impl_ops!(BitXorAssign, bitxor_assign, BitXor, bitxor for
    u8, RangeU8, &RangeU8, std::ops::RangeInclusive<u8>, SetU8, &SetU8);

impl std::ops::Not for SetU8 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        let not_set = SetU8::new();
        for i in 0..BITMAP_LEN {
            not_set.bitmap[i].set(!self.bitmap[i].get());
        }
        not_set
    }
}

impl SetU8 {
    pub(crate) fn fmt(&self, f: &mut std::fmt::Formatter<'_>, colored: bool) -> std::fmt::Result {
        if colored {
            write!(f, "{}", '['.white())?;
        } else {
            f.write_char('[')?;
        }
        let mut first = true;
        for range in self.ranges() {
            if first {
                first = false;
            } else {
                if colored {
                    write!(f, "{}", " | ".white())?;
                } else {
                    f.write_str(" | ")?;
                }
            }
            range.fmt(f, colored)?;
        }
        if colored {
            write!(f, "{}", ']'.white())
        } else {
            f.write_char(']')
        }
    }
}

impl std::fmt::Display for SetU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::fmt(self, f, false)
    }
}

impl Legible for SetU8 {
    fn legible(&self) -> impl std::fmt::Display {
        self
    }

    fn colored(&self) -> impl core::fmt::Display {
        struct Colored<'a>(&'a SetU8);
        impl core::fmt::Display for Colored<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                SetU8::fmt(self.0, f, true)
            }
        }
        Colored(self)
    }
}

pub struct ByteIter {
    set: SetU8,
    shift: u32,
}

impl ByteIter {
    fn new(set: SetU8) -> Self {
        Self { set, shift: 0 }
    }

    /// Consumes this iterator, returning the underlying [`SetU8`].
    pub fn into_set(self) -> SetU8 {
        self.set
    }
}

impl std::iter::Iterator for ByteIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.shift < BITMAP_WIDTH {
            let chunk_index = chunk_index(self.shift as u8);
            let mut chunk = self.set.bitmap[chunk_index].get();
            if chunk != 0 {
                let trailing_zeros = chunk.trailing_zeros();
                chunk &= chunk.wrapping_sub(1);
                self.set.bitmap[chunk_index].set(chunk);

                let symbol = trailing_zeros + self.shift;
                return Some(symbol as u8);
            }
            if self.shift < BITMAP_WIDTH - Chunk::BITS {
                self.shift += Chunk::BITS;
                continue;
            }
            break;
        }
        None
    }
}

pub struct RangeIter {
    set: SetU8,
    shift: u32,
    range: Option<RangeU8>,
}

impl RangeIter {
    fn new(set: SetU8) -> Self {
        Self {
            set,
            shift: 0,
            range: None,
        }
    }

    /// Consumes this iterator, returning the underlying [`SetU8`].
    pub fn into_set(self) -> SetU8 {
        self.set
    }

    #[inline]
    fn next_internal(&mut self) -> Option<RangeU8> {
        while self.shift < BITMAP_WIDTH {
            let chunk_index = chunk_index(self.shift as u8);
            let mut chunk = self.set.bitmap[chunk_index].get();
            if chunk != 0 {
                let trailing_zeros = chunk.trailing_zeros();
                chunk |= chunk.wrapping_sub(1);

                let trailing_ones = chunk.trailing_ones();
                chunk &= chunk.wrapping_add(1);

                let start = trailing_zeros + self.shift;
                let end = trailing_ones - 1 + self.shift;

                self.set.bitmap[chunk_index].set(chunk);
                return Some(RangeU8::new_unchecked(start as u8, end as u8));
            }

            if self.shift < BITMAP_WIDTH - Chunk::BITS {
                self.shift += Chunk::BITS;
                continue;
            }
            break;
        }
        None
    }
}

impl std::iter::Iterator for RangeIter {
    type Item = RangeU8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut range = self.range.take().or_else(|| self.next_internal())?;
        while let Some(next_range) = self.next_internal() {
            if range.adjoins(&next_range) {
                range = range.merge(&next_range);
            } else {
                self.range = Some(next_range);
                break;
            }
        }
        Some(range)
    }
}
