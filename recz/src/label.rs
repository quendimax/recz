#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label<'a> {
    Num(u32),
    Str(&'a str),
}

impl<'a> Label<'a> {
    pub fn is_invalid(&self) -> bool {
        matches!(self, Label::Str("") | Label::Num(u32::MAX))
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

impl<'a> From<u64> for Label<'a> {
    fn from(n: u64) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<u128> for Label<'a> {
    fn from(n: u128) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<usize> for Label<'a> {
    fn from(n: usize) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<i8> for Label<'a> {
    fn from(n: i8) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<i16> for Label<'a> {
    fn from(n: i16) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<i32> for Label<'a> {
    fn from(n: i32) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<i64> for Label<'a> {
    fn from(n: i64) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<i128> for Label<'a> {
    fn from(n: i128) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> From<isize> for Label<'a> {
    fn from(n: isize) -> Self {
        Label::Num(n.try_into().unwrap_or(u32::MAX))
    }
}

impl<'a> core::fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Label::Num(n) => write!(f, "{}", n),
            Label::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}
