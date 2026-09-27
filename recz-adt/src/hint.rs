use core::num::NonZeroUsize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LenHint {
    min: NonZeroUsize,
    max: Option<NonZeroUsize>,
}

impl LenHint {
    pub const fn new(min: usize, max: Option<usize>) -> Self {
        Self {
            min: into_non_zero(min),
            max: if let Some(max) = max {
                Some(into_non_zero(max))
            } else {
                None
            },
        }
    }

    pub fn min(&self) -> usize {
        self.min.get().wrapping_sub(1)
    }

    pub fn max(&self) -> Option<usize> {
        self.max.map(|v| v.get().wrapping_sub(1))
    }

    pub fn exact_len(&self) -> Option<usize> {
        if self.max == Some(self.min) {
            self.max()
        } else {
            None
        }
    }

    pub fn to_tuple(&self) -> (usize, Option<usize>) {
        (self.min(), self.max())
    }
}

impl core::convert::From<(usize, Option<usize>)> for LenHint {
    fn from((min, max): (usize, Option<usize>)) -> Self {
        Self::new(min, max)
    }
}

impl core::ops::BitAnd for LenHint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let min = NonZeroUsize::new(self.min.get() + rhs.min.get().wrapping_sub(1))
            .expect("`LenHint`'s min overflow");
        let max = match (self.max, rhs.max) {
            (Some(a), Some(b)) => Some(
                NonZeroUsize::new(a.get() + b.get().wrapping_sub(1))
                    .expect("`LenHint`'s max overflow"),
            ),
            _ => None,
        };
        Self { min, max }
    }
}

impl core::ops::BitAndAssign for LenHint {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = core::ops::BitAnd::bitand(*self, rhs);
    }
}

impl core::ops::BitOr for LenHint {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min.min(rhs.min),
            max: match (self.max, rhs.max) {
                (Some(a), Some(b)) => Some(a.max(b)),
                _ => None,
            },
        }
    }
}

impl core::ops::BitOrAssign for LenHint {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = core::ops::BitOr::bitor(*self, rhs);
    }
}

impl core::ops::Mul<usize> for LenHint {
    type Output = Self;

    fn mul(self, multiplier: usize) -> Self::Output {
        Self::new(self.min() * multiplier, self.max().map(|v| v * multiplier))
    }
}

impl core::ops::MulAssign<usize> for LenHint {
    fn mul_assign(&mut self, multiplier: usize) {
        *self = core::ops::Mul::mul(*self, multiplier);
    }
}

impl core::ops::Mul for LenHint {
    type Output = Self;

    fn mul(self, multiplier: Self) -> Self::Output {
        Self {
            min: into_non_zero(self.min() * multiplier.min()),
            max: match (self.max(), multiplier.max()) {
                (Some(a), Some(b)) => Some(into_non_zero(a * b)),
                _ => None,
            },
        }
    }
}

impl core::cmp::PartialEq<(usize, Option<usize>)> for LenHint {
    fn eq(&self, other: &(usize, Option<usize>)) -> bool {
        self.to_tuple() == *other
    }
}

impl core::fmt::Debug for LenHint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_tuple().fmt(f)
    }
}

const fn into_non_zero(v: usize) -> NonZeroUsize {
    NonZeroUsize::new(v.wrapping_add(1)).expect("usize::MAX is not allowed value for `LenHint`")
}
