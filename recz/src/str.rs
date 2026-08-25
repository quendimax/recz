use crate::CaptureLabel;
use core::iter::Iterator;
use core::range::Range;

pub trait Capture<'h> {
    fn as_str(&self) -> &'h str;

    fn start(&self) -> usize;

    fn end(&self) -> usize;

    #[inline]
    fn span(&self) -> Range<usize> {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.end() - self.start()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Match<'h>: Capture<'h> {
    fn haystack(&self) -> &'h str;
    fn group<'a>(&self, label: impl Into<CaptureLabel<'a>>) -> Option<impl Capture<'h>>;
    fn groups(&self) -> impl Iterator<Item = impl Capture<'h>> + '_;
}

pub trait Regex {
    fn pattern(&self) -> &'static str;
    fn find<'h>(&self, haystack: &'h str) -> Option<impl Match<'h>>;
}
