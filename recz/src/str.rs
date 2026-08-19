pub trait Capture<'h> {
    fn as_str(&self) -> &'h str;

    fn start(&self) -> usize;

    fn end(&self) -> usize;

    #[inline]
    fn span(&self) -> core::range::Range<usize> {
        core::range::Range {
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
    fn group(&self, id: u32) -> Option<impl Capture<'h>>;
    fn groups(&self) -> impl core::iter::Iterator<Item = impl Capture<'h>> + '_;
}

pub trait Regex {
    fn pattern(&self) -> &'static str;
    fn find<'h>(&self, haystack: &'h str) -> Option<impl Match<'h>>;

    #[inline]
    fn is_match(&self, haystack: &str) -> bool {
        self.find(haystack).is_some()
    }
}
