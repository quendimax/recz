use crate::{Legible, RangeU8, Step};
use std::cell::Cell;
use std::fmt::Write;
use std::ops::Deref;
use std::ops::RangeInclusive;

type Chunk = u64;

/// Quantity of `Chunk` values in the `chunks` member for symbols' bits.
const BITMAP_LEN: usize = (u8::MAX as usize + 1) / Chunk::BITS as usize;

/// A set of symbols that can be used to represent any byte.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetU8 {
    chunks: [Cell<Chunk>; BITMAP_LEN],
}

impl SetU8 {
    /// Creates a new empty symbol set.
    #[inline]
    pub const fn new() -> Self {
        Self {
            chunks: [Cell::new(0), Cell::new(0), Cell::new(0), Cell::new(0)],
        }
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
        let mut set = Self::new();
        set |= value;
        set
    }
}

impl std::convert::From<std::ops::RangeInclusive<u8>> for SetU8 {
    fn from(value: std::ops::RangeInclusive<u8>) -> Self {
        let mut set = Self::new();
        set |= RangeU8::from(value);
        set
    }
}

impl std::convert::From<RangeU8> for SetU8 {
    fn from(value: RangeU8) -> Self {
        let mut set = Self::new();
        set |= value;
        set
    }
}

impl std::convert::From<&RangeU8> for SetU8 {
    fn from(value: &RangeU8) -> Self {
        let mut set = Self::new();
        set |= value;
        set
    }
}

impl std::convert::From<&[u8]> for SetU8 {
    fn from(value: &[u8]) -> Self {
        let mut set = Self::default();
        for byte in value {
            set |= *byte;
        }
        set
    }
}

impl<const N: usize> std::convert::From<&[u8; N]> for SetU8 {
    fn from(value: &[u8; N]) -> Self {
        std::convert::From::<&[u8]>::from(&value[..])
    }
}

impl std::convert::AsRef<SetU8> for SetU8 {
    #[inline]
    fn as_ref(&self) -> &SetU8 {
        self
    }
}

impl std::hash::Hash for SetU8 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.chunks.iter().for_each(|chunk| chunk.get().hash(state));
    }
}

macro_rules! impl_ops {
    ($op_assign_trait:ident, $op_assign_fn:ident, $op_trait:ident, $op_fn:ident) => {
        impl ::std::ops::$op_assign_trait<&SetU8> for SetU8 {
            #[inline]
            fn $op_assign_fn(&mut self, rhs: &SetU8) {
                use ::std::ops::$op_trait;
                self.chunks[0].update(|chunk| chunk.$op_fn(&rhs.chunks[0].get()));
                self.chunks[1].update(|chunk| chunk.$op_fn(&rhs.chunks[1].get()));
                self.chunks[2].update(|chunk| chunk.$op_fn(&rhs.chunks[2].get()));
                self.chunks[3].update(|chunk| chunk.$op_fn(&rhs.chunks[3].get()));
            }
        }

        impl ::std::ops::$op_assign_trait<SetU8> for SetU8 {
            #[inline]
            fn $op_assign_fn(&mut self, rhs: SetU8) {
                self.$op_assign_fn(&rhs);
            }
        }

        impl ::std::ops::$op_assign_trait<&RangeU8> for SetU8 {
            #[inline]
            fn $op_assign_fn(&mut self, range: &RangeU8) {
                use ::std::ops::$op_trait;
                let (ls_mask, ms_mask, ls_index, ms_index) = find_masks_indices(*range);
                unsafe {
                    match ms_index - ls_index {
                        0 => {
                            let mask = ls_mask & ms_mask;
                            self.chunks
                                .get_unchecked_mut(ls_index)
                                .update(|chunk| chunk.$op_fn(mask));
                        }
                        1 => {
                            self.chunks
                                .get_unchecked_mut(ls_index)
                                .update(|chunk| chunk.$op_fn(ls_mask));
                            self.chunks
                                .get_unchecked_mut(ls_index + 1)
                                .update(|chunk| chunk.$op_fn(ms_mask));
                        }
                        2 => {
                            self.chunks
                                .get_unchecked_mut(ls_index)
                                .update(|chunk| chunk.$op_fn(ls_mask));
                            self.chunks
                                .get_unchecked_mut(ls_index + 1)
                                .update(|chunk| chunk.$op_fn(Chunk::MAX));
                            self.chunks
                                .get_unchecked_mut(ls_index + 2)
                                .update(|chunk| chunk.$op_fn(ms_mask));
                        }
                        3 => {
                            self.chunks
                                .get_unchecked_mut(0)
                                .update(|chunk| chunk.$op_fn(ls_mask));
                            self.chunks
                                .get_unchecked_mut(1)
                                .update(|chunk| chunk.$op_fn(Chunk::MAX));
                            self.chunks
                                .get_unchecked_mut(2)
                                .update(|chunk| chunk.$op_fn(Chunk::MAX));
                            self.chunks
                                .get_unchecked_mut(3)
                                .update(|chunk| chunk.$op_fn(ms_mask));
                        }
                        _ => std::hint::unreachable_unchecked(),
                    };
                };
            }
        }

        impl ::std::ops::$op_assign_trait<RangeU8> for SetU8 {
            #[inline]
            fn $op_assign_fn(&mut self, rhs: RangeU8) {
                self.$op_assign_fn(&rhs);
            }
        }

        impl ::std::ops::$op_assign_trait<&u8> for SetU8 {
            #[inline]
            fn $op_assign_fn(&mut self, byte: &u8) {
                use ::std::ops::$op_trait;
                let bit = 1 << (*byte & (u8::MAX >> 2));
                self.chunks[*byte as usize >> 6].update(|ch| ch.$op_fn(bit));
            }
        }

        impl ::std::ops::$op_assign_trait<u8> for SetU8 {
            #[inline]
            fn $op_assign_fn(&mut self, byte: u8) {
                self.$op_assign_fn(&byte);
            }
        }

        impl ::std::ops::$op_trait<SetU8> for SetU8 {
            type Output = Self;

            #[inline]
            fn $op_fn(self, rhs: SetU8) -> Self {
                let mut result = self.clone();
                ::std::ops::$op_assign_trait::$op_assign_fn(&mut result, &rhs);
                result
            }
        }

        impl ::std::ops::$op_trait<RangeU8> for SetU8 {
            type Output = Self;

            #[inline]
            fn $op_fn(self, rhs: RangeU8) -> Self {
                let mut result = self.clone();
                ::std::ops::$op_assign_trait::$op_assign_fn(&mut result, &rhs);
                result
            }
        }

        impl ::std::ops::$op_trait<u8> for SetU8 {
            type Output = Self;

            #[inline]
            fn $op_fn(self, rhs: u8) -> Self {
                let mut result = self.clone();
                ::std::ops::$op_assign_trait::$op_assign_fn(&mut result, &rhs);
                result
            }
        }
    };
}

impl_ops!(BitAndAssign, bitand_assign, BitAnd, bitand);
impl_ops!(BitOrAssign, bitor_assign, BitOr, bitor);
impl_ops!(BitXorAssign, bitxor_assign, BitXor, bitxor);

impl std::ops::Not for SetU8 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self {
            chunks: [
                Cell::new(!self.chunks[0].get()),
                Cell::new(!self.chunks[1].get()),
                Cell::new(!self.chunks[2].get()),
                Cell::new(!self.chunks[3].get()),
            ],
        }
    }
}

impl crate::ops::Containable<u8> for SetU8 {
    #[inline]
    fn contains(&self, byte: u8) -> bool {
        self.chunks[byte as usize >> 6].get() & (1 << (byte & (u8::MAX >> 2))) != 0
    }
}

impl crate::ops::Containable<RangeU8> for SetU8 {
    fn contains(&self, range: RangeU8) -> bool {
        let (ls_mask, ms_mask, ls_index, ms_index) = find_masks_indices(range);
        unsafe {
            match ms_index - ls_index {
                0 => {
                    let mask = ls_mask & ms_mask;
                    self.chunks.get_unchecked(ls_index).get() & mask == mask
                }
                1 => {
                    self.chunks.get_unchecked(ls_index).get() & ls_mask == ls_mask
                        && self.chunks.get_unchecked(ls_index + 1).get() & ms_mask == ms_mask
                }
                2 => {
                    self.chunks.get_unchecked(ls_index).get() & ls_mask == ls_mask
                        && self.chunks.get_unchecked(ls_index + 1).get() == Chunk::MAX
                        && self.chunks.get_unchecked(ls_index + 2).get() & ms_mask == ms_mask
                }
                3 => {
                    self.chunks.get_unchecked(0).get() & ls_mask == ls_mask
                        && self.chunks.get_unchecked(1).get() == Chunk::MAX
                        && self.chunks.get_unchecked(2).get() == Chunk::MAX
                        && self.chunks.get_unchecked(3).get() & ms_mask == ms_mask
                }
                _ => std::hint::unreachable_unchecked(),
            }
        }
    }
}

impl crate::ops::Containable<&SetU8> for SetU8 {
    #[inline]
    fn contains(&self, rhs: &SetU8) -> bool {
        self.chunks[0].get() & rhs.chunks[0].get() == rhs.chunks[0].get()
            && self.chunks[1].get() & rhs.chunks[1].get() == rhs.chunks[1].get()
            && self.chunks[2].get() & rhs.chunks[2].get() == rhs.chunks[2].get()
            && self.chunks[3].get() & rhs.chunks[3].get() == rhs.chunks[3].get()
    }
}

impl crate::ops::Containable for SetU8 {
    #[inline]
    fn contains(&self, rhs: Self) -> bool {
        self.contains(&rhs)
    }
}

impl crate::ops::Intersectable<u8> for SetU8 {
    #[inline]
    fn intersects(&self, byte: u8) -> bool {
        self.chunks[byte as usize >> 6].get() & (1 << (byte & (u8::MAX >> 2))) != 0
    }
}

impl crate::ops::Intersectable<RangeU8> for SetU8 {
    fn intersects(&self, range: RangeU8) -> bool {
        let (ls_mask, ms_mask, ls_index, ms_index) = find_masks_indices(range);
        unsafe {
            match ms_index - ls_index {
                0 => {
                    let mask = ls_mask & ms_mask;
                    self.chunks.get_unchecked(ls_index).get() & mask != 0
                }
                1 => {
                    self.chunks.get_unchecked(ls_index).get() & ls_mask != 0
                        || self.chunks.get_unchecked(ls_index + 1).get() & ms_mask != 0
                }
                2 => {
                    self.chunks.get_unchecked(ls_index).get() & ls_mask != 0
                        || self.chunks.get_unchecked(ls_index + 1).get() != 0
                        || self.chunks.get_unchecked(ls_index + 2).get() & ms_mask != 0
                }
                3 => {
                    self.chunks.get_unchecked(0).get() & ls_mask != 0
                        || self.chunks.get_unchecked(1).get() != 0
                        || self.chunks.get_unchecked(2).get() != 0
                        || self.chunks.get_unchecked(3).get() & ms_mask != 0
                }
                _ => std::hint::unreachable_unchecked(),
            }
        }
    }
}

impl crate::ops::Intersectable<&SetU8> for SetU8 {
    #[inline]
    fn intersects(&self, rhs: &SetU8) -> bool {
        self.chunks[0].get() & rhs.chunks[0].get() != 0
            || self.chunks[1].get() & rhs.chunks[1].get() != 0
            || self.chunks[2].get() & rhs.chunks[2].get() != 0
            || self.chunks[3].get() & rhs.chunks[3].get() != 0
    }
}

impl crate::ops::Intersectable for SetU8 {
    #[inline]
    fn intersects(&self, rhs: Self) -> bool {
        self.intersects(&rhs)
    }
}

impl crate::ops::Includable<u8> for SetU8 {
    #[inline]
    fn include(&mut self, byte: u8) -> &mut Self {
        *self |= byte;
        self
    }
}

impl crate::ops::Includable<RangeU8> for SetU8 {
    #[inline]
    fn include(&mut self, range: RangeU8) -> &mut Self {
        *self |= range;
        self
    }
}

impl crate::ops::Includable<RangeInclusive<u8>> for SetU8 {
    #[inline]
    fn include(&mut self, range: RangeInclusive<u8>) -> &mut Self {
        *self |= RangeU8::from(range);
        self
    }
}

impl crate::ops::Includable<&SetU8> for SetU8 {
    #[inline]
    fn include(&mut self, rhs: &SetU8) -> &mut Self {
        *self |= rhs;
        self
    }
}

impl crate::ops::Includable for SetU8 {
    #[inline]
    fn include(&mut self, rhs: SetU8) -> &mut Self {
        *self |= rhs;
        self
    }
}

impl crate::ops::Excludable<u8> for SetU8 {
    #[inline]
    fn exclude(&mut self, byte: u8) -> &mut Self {
        self.chunks[byte as usize >> 6].update(|ch| ch & !(1 << (byte & (u8::MAX >> 2))));
        self
    }
}

impl crate::ops::Excludable<RangeU8> for SetU8 {
    #[inline]
    fn exclude(&mut self, range: RangeU8) -> &mut Self {
        let (ls_mask, ms_mask, ls_index, ms_index) = find_masks_indices(range);
        unsafe {
            match ms_index - ls_index {
                0 => {
                    self.chunks
                        .get_unchecked_mut(ls_index)
                        .update(|ch| ch & !(ls_mask & ms_mask));
                }
                1 => {
                    self.chunks
                        .get_unchecked_mut(ls_index)
                        .update(|ch| ch & !ls_mask);
                    self.chunks
                        .get_unchecked_mut(ls_index + 1)
                        .update(|ch| ch & !ms_mask);
                }
                2 => {
                    self.chunks
                        .get_unchecked_mut(ls_index)
                        .update(|ch| ch & !ls_mask);
                    self.chunks
                        .get_unchecked_mut(ls_index + 1)
                        .update(|ch| ch & !Chunk::MAX);
                    self.chunks
                        .get_unchecked_mut(ls_index + 2)
                        .update(|ch| ch & !ms_mask);
                }
                3 => {
                    self.chunks.get_unchecked_mut(0).update(|ch| ch & !ls_mask);
                    self.chunks
                        .get_unchecked_mut(1)
                        .update(|ch| ch & !Chunk::MAX);
                    self.chunks
                        .get_unchecked_mut(2)
                        .update(|ch| ch & !Chunk::MAX);
                    self.chunks.get_unchecked_mut(3).update(|ch| ch & !ms_mask);
                }
                _ => std::hint::unreachable_unchecked(),
            }
        }
        self
    }
}

impl crate::ops::Excludable<RangeInclusive<u8>> for SetU8 {
    fn exclude(&mut self, rhs: RangeInclusive<u8>) -> &mut Self {
        self.exclude(RangeU8::from(rhs));
        self
    }
}

impl crate::ops::Excludable<&SetU8> for SetU8 {
    #[inline]
    fn exclude(&mut self, rhs: &SetU8) -> &mut Self {
        *self &= !rhs.clone();
        self
    }
}

impl crate::ops::Excludable for SetU8 {
    #[inline]
    fn exclude(&mut self, rhs: SetU8) -> &mut Self {
        *self &= !rhs;
        self
    }
}

impl SetU8 {
    /// Checks if the set is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.chunks.iter().all(|chunk| chunk.get() == 0)
    }

    /// Returns an iterator over the bytes in the set.
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        ByteIter::new(self)
    }

    /// Returns an iterator over the inclusive byte ranges in the set.
    pub fn ranges(&self) -> impl Iterator<Item = RangeU8> {
        RangeIter::new(self)
    }
}

impl std::fmt::Display for SetU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        let mut iter = self.ranges();
        let mut range = iter.next();
        while let Some(cur_range) = range {
            if let Some(next_range) = iter.next() {
                if cur_range.last().steps_between(next_range.start()) == 1 {
                    range = Some(RangeU8::new(cur_range.start(), next_range.last()));
                    continue;
                } else {
                    std::fmt::Display::fmt(&cur_range.display(), f)?;
                    f.write_str(" | ")?;
                    range = Some(next_range);
                }
            } else {
                std::fmt::Display::fmt(&cur_range.display(), f)?;
                break;
            }
        }
        f.write_char(']')
    }
}

pub struct ByteIter<T> {
    set: T,
    chunk: Chunk,
    shift: u32,
}

impl<T> ByteIter<T>
where
    T: Deref<Target = SetU8>,
{
    pub fn new(set: T) -> Self {
        let chunk = set.chunks[0].get();
        Self {
            set,
            chunk,
            shift: 0,
        }
    }
}

impl<T> std::iter::Iterator for ByteIter<T>
where
    T: Deref<Target = SetU8>,
{
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        const SHIFT_OVERFLOW: u32 = (BITMAP_LEN << 6) as u32;
        while self.shift < SHIFT_OVERFLOW {
            if self.chunk != 0 {
                let trailing_zeros = self.chunk.trailing_zeros();
                self.chunk &= self.chunk.wrapping_sub(1);
                let symbol = trailing_zeros + self.shift;
                return Some(symbol as u8);
            }
            if self.shift < SHIFT_OVERFLOW - 64 {
                self.shift += 64;
                self.chunk = self.set.chunks[self.shift as usize >> 6].get();
                continue;
            }
            break;
        }
        None
    }
}

pub struct RangeIter<T> {
    set: T,
    chunk: Chunk,
    shift: u32,
}

impl<T> RangeIter<T>
where
    T: Deref<Target = SetU8>,
{
    pub fn new(set: T) -> Self {
        let chunk = set.chunks[0].get();
        Self {
            set,
            chunk,
            shift: 0,
        }
    }
}

impl<T> std::iter::Iterator for RangeIter<T>
where
    T: Deref<Target = SetU8>,
{
    type Item = RangeU8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        const SHIFT_OVERFLOW: u32 = (BITMAP_LEN << 6) as u32;
        while self.shift < SHIFT_OVERFLOW {
            if self.chunk != 0 {
                let trailing_zeros = self.chunk.trailing_zeros();
                self.chunk |= self.chunk.wrapping_sub(1);

                let trailing_ones = self.chunk.trailing_ones();
                self.chunk &= self.chunk.wrapping_add(1);

                let start = trailing_zeros + self.shift;
                let end = trailing_ones - 1 + self.shift;

                return Some(RangeU8::new_unchecked(start as u8, end as u8));
            }

            if self.shift < SHIFT_OVERFLOW - 64 {
                self.shift += 64;
                self.chunk = self.set.chunks[self.shift as usize >> 6].get();
                continue;
            }
            break;
        }
        None
    }
}

fn find_masks_indices(range: RangeU8) -> (u64, u64, usize, usize) {
    let mut ls_mask = 1 << (range.start() & (u8::MAX >> 2));
    ls_mask = !(ls_mask - 1);

    let mut ms_mask = 1 << (range.last() & (u8::MAX >> 2));
    ms_mask |= ms_mask - 1;

    let ls_index = (range.start() >> 6) as usize;
    let ms_index = (range.last() >> 6) as usize;

    (ls_mask, ms_mask, ls_index, ms_index)
}
