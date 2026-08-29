use core::range::Range;

pub trait Capture<'h> {
    fn as_str(&self) -> &'h str;

    fn start(&self) -> usize;

    fn end(&self) -> usize;

    #[inline]
    fn range(&self) -> Range<usize> {
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
    fn capture<'a>(&self, name: &'a str) -> Option<impl Capture<'h>>;
    fn capture_by_num(&self, number: u32) -> Option<impl Capture<'h>>;
}

pub trait Regex {
    fn pattern(&self) -> &'static str;
    fn find<'h>(&self, haystack: &'h str) -> Option<impl Match<'h>>;
}
