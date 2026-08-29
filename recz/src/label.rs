#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label<'a> {
    Num(u32),
    Str(&'a str),
}

impl<'a> Label<'a> {
    pub const fn invalid() -> Self {
        Self::Str("")
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, Label::Str(""))
    }
}

impl<'a> From<&'a str> for Label<'a> {
    fn from(s: &'a str) -> Self {
        Label::Str(s)
    }
}

impl<'a> From<u8> for Label<'a> {
    fn from(n: u8) -> Self {
        Label::Num(n.into())
    }
}

impl<'a> From<u16> for Label<'a> {
    fn from(n: u16) -> Self {
        Label::Num(n.into())
    }
}

impl<'a> From<u32> for Label<'a> {
    fn from(n: u32) -> Self {
        Label::Num(n)
    }
}

macro_rules! impl_from {
    ($($ty:ty),* $(,)?) => {$(
        impl<'a> From<$ty> for Label<'a> {
            #[inline]
            fn from(n: $ty) -> Self {
                if let Ok(n) = n.try_into() {
                    Label::Num(n)
                } else {
                    Label::invalid()
                }
            }
        }
    )*};
}
impl_from!(u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<'a> core::fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Label::Num(n) => n.fmt(f),
            Label::Str(s) => s.fmt(f),
        }
    }
}
