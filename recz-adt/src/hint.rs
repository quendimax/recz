use core::num::NonZeroUsize;

/// Lower and upper bounds on a size. An upper bound of `None` means the size is
/// unbounded.
///
/// Bounds must not exceed `usize::MAX - 1`; constructing or computing a larger
/// one panics.
///
/// # Examples
///
/// ```
/// use recz_adt::SizeHint;
///
/// let ab = SizeHint::new(2, Some(2));
/// let digits = SizeHint::new(1, None);
///
/// assert_eq!(ab & digits, (3, None));
/// assert_eq!(ab | SizeHint::new(0, Some(5)), (0, Some(5)));
/// assert_eq!(ab * 3, (6, Some(6)));
/// assert_eq!(ab.exact_size(), Some(2));
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SizeHint {
    least: NonZeroUsize,
    most: Option<NonZeroUsize>,
}

impl SizeHint {
    pub const fn new(min: usize, max: Option<usize>) -> Self {
        Self {
            least: into_non_zero(min),
            most: if let Some(max) = max {
                Some(into_non_zero(max))
            } else {
                None
            },
        }
    }

    pub fn least(&self) -> usize {
        self.least.get().wrapping_sub(1)
    }

    pub fn most(&self) -> Option<usize> {
        self.most.map(|v| v.get().wrapping_sub(1))
    }

    /// Returns `Some(size)` if this hint has an exact size, otherwise returns
    /// `None`.
    pub fn exact_size(&self) -> Option<usize> {
        if self.most == Some(self.least) {
            self.most()
        } else {
            None
        }
    }

    pub fn to_tuple(&self) -> (usize, Option<usize>) {
        (self.least(), self.most())
    }
}

impl core::convert::From<(usize, Option<usize>)> for SizeHint {
    fn from((min, max): (usize, Option<usize>)) -> Self {
        Self::new(min, max)
    }
}

impl core::ops::BitAnd for SizeHint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let min = NonZeroUsize::new(self.least.get() + rhs.least.get().wrapping_sub(1))
            .expect("`LenHint`'s min overflow");
        let max = match (self.most, rhs.most) {
            (Some(a), Some(b)) => Some(
                NonZeroUsize::new(a.get() + b.get().wrapping_sub(1))
                    .expect("`LenHint`'s max overflow"),
            ),
            _ => None,
        };
        Self {
            least: min,
            most: max,
        }
    }
}

impl core::ops::BitAndAssign for SizeHint {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = core::ops::BitAnd::bitand(*self, rhs);
    }
}

impl core::ops::BitOr for SizeHint {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            least: self.least.min(rhs.least),
            most: match (self.most, rhs.most) {
                (Some(a), Some(b)) => Some(a.max(b)),
                _ => None,
            },
        }
    }
}

impl core::ops::BitOrAssign for SizeHint {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = core::ops::BitOr::bitor(*self, rhs);
    }
}

impl core::ops::Mul<usize> for SizeHint {
    type Output = Self;

    fn mul(self, multiplier: usize) -> Self::Output {
        Self::new(
            self.least() * multiplier,
            self.most().map(|v| v * multiplier),
        )
    }
}

impl core::ops::MulAssign<usize> for SizeHint {
    fn mul_assign(&mut self, multiplier: usize) {
        *self = core::ops::Mul::mul(*self, multiplier);
    }
}

impl core::ops::Mul for SizeHint {
    type Output = Self;

    fn mul(self, multiplier: Self) -> Self::Output {
        Self {
            least: into_non_zero(self.least() * multiplier.least()),
            most: match (self.most(), multiplier.most()) {
                (Some(a), Some(b)) => Some(into_non_zero(a * b)),
                _ => None,
            },
        }
    }
}

impl core::ops::MulAssign for SizeHint {
    fn mul_assign(&mut self, multiplier: Self) {
        *self = core::ops::Mul::mul(*self, multiplier);
    }
}

impl core::cmp::PartialEq<(usize, Option<usize>)> for SizeHint {
    fn eq(&self, other: &(usize, Option<usize>)) -> bool {
        self.to_tuple() == *other
    }
}

impl core::fmt::Debug for SizeHint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_tuple().fmt(f)
    }
}

const fn into_non_zero(v: usize) -> NonZeroUsize {
    NonZeroUsize::new(v.wrapping_add(1)).expect("usize::MAX is not allowed value for `LenHint`")
}
