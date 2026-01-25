use crate::ops::*;
use crate::symbol::Epsilon;
use crate::tag::Inst;
use redt::{ByteIter, Legible, RangeIter, RangeU8, SetU8, Step};
use std::cell::{Ref, RefCell};
use std::fmt::Write;
use std::ops::Deref;
use std::ptr::NonNull;

/// Transition is a struct that contains symbols that connect two nodes. The
/// symbols can be bytes and Epsilon.
///
/// # Implementation
///
/// Symbols are the corresponding bits in `chunks` bitmap from 4x`Chunk` values.
/// The 256-th bit is for Epsilon.
pub struct Transition<'a>(&'a TransInner);

pub(crate) type TransPtr = NonNull<TransInner>;

pub(crate) struct TransInner {
    symset: RefCell<SetU8>,
    inst: Inst,
}

/// Crate API
impl<'a> Transition<'a> {
    /// Creates a new empty transition.
    #[inline(always)]
    pub(crate) fn new_inner(inst: Inst) -> TransInner {
        TransInner {
            symset: RefCell::new(SetU8::empty()),
            inst,
        }
    }

    #[inline(always)]
    pub(crate) fn as_ptr(&self) -> TransPtr {
        NonNull::from(self.0)
    }
}

impl<'a> Transition<'a> {
    /// Checks if these transitions are two references to the same transition.
    #[inline]
    pub fn is(self, other: Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }

    #[inline]
    pub fn is_epsilon(&self) -> bool {
        self.0.symset.borrow().is_empty()
    }

    /// Returns iterator over all symbols in this trasition instance in
    /// ascendent order.
    pub fn symbols(self) -> impl Iterator<Item = u8> {
        let borrow = self.0.symset.borrow();
        ByteIter::new(borrow)
    }

    /// Returns iterator over all symbol ranges in this trasition instance in
    /// ascendent order.
    pub fn ranges(self) -> impl Iterator<Item = RangeU8> {
        let borrow = self.0.symset.borrow();
        RangeIter::new(borrow)
    }

    /// Returns a clone of the symbol set in this transition instance.
    pub fn as_set(&self) -> Ref<'_, SetU8> {
        self.0.symset.borrow()
    }

    #[inline]
    pub fn instruct(self) -> Inst {
        self.0.inst
    }

    /// Merges the `other` object into this transition.
    pub fn merge<T>(&self, other: T)
    where
        Self: Mergeable<T>,
    {
        Mergeable::merge(self, other);
    }

    pub fn intersects<T>(&self, other: T) -> bool
    where
        Self: Intersectable<T>,
    {
        Intersectable::intersects(self, other)
    }

    pub fn contains<T>(&self, other: T) -> bool
    where
        Self: Containable<T>,
    {
        Containable::contains(self, other)
    }
}

impl<T> Containable<T> for Transition<'_>
where
    SetU8: Containable<T>,
{
    fn contains(&self, rhs: T) -> bool {
        self.0.symset.borrow().contains(rhs)
    }
}

impl Containable<Epsilon> for Transition<'_> {
    fn contains(&self, _: Epsilon) -> bool {
        self.0.symset.borrow().is_empty()
    }
}

impl<'a, 'b> Containable<Transition<'b>> for Transition<'a> {
    #[inline]
    fn contains(&self, other: Transition<'b>) -> bool {
        self.0
            .symset
            .borrow()
            .contains(other.0.symset.borrow().deref())
    }
}

impl<'a, 'b> Containable<&Transition<'b>> for Transition<'a> {
    #[inline]
    fn contains(&self, other: &Transition<'b>) -> bool {
        Containable::contains(self, *other)
    }
}

impl<T> Intersectable<T> for Transition<'_>
where
    SetU8: Intersectable<T>,
{
    fn intersects(&self, rhs: T) -> bool {
        self.0.symset.borrow().intersects(rhs)
    }
}

impl Intersectable<Epsilon> for Transition<'_> {
    fn intersects(&self, _: Epsilon) -> bool {
        self.0.symset.borrow().is_empty()
    }
}

impl<'a, 'b> redt::ops::Intersectable<Transition<'b>> for Transition<'a> {
    #[inline]
    fn intersects(&self, other: Transition<'b>) -> bool {
        self.0
            .symset
            .borrow()
            .intersects(other.0.symset.borrow().deref())
    }
}

impl<'a, 'b> Intersectable<&Transition<'b>> for Transition<'a> {
    #[inline]
    fn intersects(&self, other: &Transition<'b>) -> bool {
        Intersectable::intersects(self, *other)
    }
}

impl<T> Mergeable<T> for Transition<'_>
where
    SetU8: Includable<T>,
{
    fn merge(&self, rhs: T) -> &Self {
        self.0.symset.borrow_mut().include(rhs);
        self
    }
}

impl<'a, 'b> Mergeable<Transition<'b>> for Transition<'a> {
    fn merge(&self, other: Transition<'b>) -> &Self {
        let other_symset = other.0.symset.borrow();
        let other_symset = other_symset.deref();
        self.0.symset.borrow_mut().include(other_symset);
        self
    }
}

impl<'a, 'b> Mergeable<&Transition<'b>> for Transition<'a> {
    fn merge(&self, other: &Transition<'b>) -> &Self {
        Mergeable::merge(self, *other);
        self
    }
}

impl<T> Rejectable<T> for Transition<'_>
where
    T: Clone,
    SetU8: Excludable<T>,
{
    fn reject(&self, rhs: T) -> &Self {
        self.0.symset.borrow_mut().exclude(rhs);
        self
    }
}

impl Copy for Transition<'_> {}

impl Clone for Transition<'_> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> std::convert::From<&'a TransInner> for Transition<'a> {
    fn from(inner: &'a TransInner) -> Self {
        Self(inner)
    }
}

impl std::cmp::Eq for Transition<'_> {}

impl std::cmp::PartialEq for Transition<'_> {
    /// Tests equality between symbols only, not instructions.
    fn eq(&self, other: &Self) -> bool {
        self.0.symset.borrow().eq(other.0.symset.borrow().deref())
    }
}

impl std::fmt::Display for Transition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // symbols
        f.write_char('[')?;
        let mut iter = self.ranges();
        let mut range = iter.next();
        let mut has_symbols = false;
        while let Some(cur_range) = range {
            has_symbols = true;
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
        if self.contains(Epsilon) {
            if has_symbols {
                f.write_str(" | ")?;
            }
            f.write_str("Epsilon")?;
        }
        f.write_char(']')?;

        // instruction
        if self.0.inst != Inst::Nop {
            f.write_str(" / `")?;
            std::fmt::Display::fmt(&self.0.inst, f)?;
            f.write_char('`')?;
        }
        Ok(())
    }
}

macro_rules! impl_fmt {
    (std::fmt::$trait:ident) => {
        impl std::fmt::$trait for Transition<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_char('[')?;
                let mut first_iter = true;
                for range in self.ranges() {
                    if first_iter {
                        first_iter = false;
                    } else {
                        f.write_str(" | ")?;
                    }
                    ::std::fmt::$trait::fmt(&range, f)?;
                }

                if self.contains(Epsilon) {
                    if !first_iter {
                        f.write_str(" | ")?;
                    }
                    f.write_str("Epsilon")?;
                }
                f.write_char(']')
            }
        }
    };
}

impl_fmt!(std::fmt::Debug);
impl_fmt!(std::fmt::Binary);
impl_fmt!(std::fmt::Octal);
impl_fmt!(std::fmt::LowerHex);
impl_fmt!(std::fmt::UpperHex);
