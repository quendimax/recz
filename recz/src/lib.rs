#![cfg_attr(not(test), no_std)]

pub mod str;

pub use recz_macro::__re as re;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureLabel<'a> {
    Digit(u32),
    Name(&'a str),
}

impl core::convert::From<u32> for CaptureLabel<'_> {
    #[inline]
    fn from(num: u32) -> Self {
        CaptureLabel::Digit(num)
    }
}

impl core::convert::From<u16> for CaptureLabel<'_> {
    #[inline]
    fn from(num: u16) -> Self {
        CaptureLabel::Digit(num.into())
    }
}

impl core::convert::From<u8> for CaptureLabel<'_> {
    #[inline]
    fn from(num: u8) -> Self {
        CaptureLabel::Digit(num.into())
    }
}

impl<'a, T> core::convert::From<&'a T> for CaptureLabel<'a>
where
    T: AsRef<str> + ?Sized,
{
    #[inline]
    fn from(name: &'a T) -> Self {
        CaptureLabel::Name(name.as_ref())
    }
}
