use ::core::iter::Iterator;
use ::core::range::Range;
use ::recz::CaptureLabel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capture<'h> {
    capture: &'h str,
    start: usize,
}

impl<'h> ::recz::str::Capture<'h> for Capture<'h> {
    #[inline]
    fn as_str(&self) -> &'h str {
        self.capture
    }

    #[inline]
    fn start(&self) -> usize {
        self.start
    }

    #[inline]
    fn end(&self) -> usize {
        self.start + self.capture.len()
    }
}

impl<'h> Capture<'h> {
    #[inline]
    pub fn as_str(&self) -> &'h str {
        ::recz::str::Capture::as_str(self)
    }

    #[inline]
    pub fn start(&self) -> usize {
        ::recz::str::Capture::start(self)
    }

    #[inline]
    pub fn end(&self) -> usize {
        ::recz::str::Capture::end(self)
    }

    #[inline]
    pub fn span(&self) -> Range<usize> {
        ::recz::str::Capture::span(self)
    }

    #[inline]
    pub fn len(&self) -> usize {
        ::recz::str::Capture::len(self)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        ::recz::str::Capture::is_empty(self)
    }
}

const GROUP_COUNT: usize = 3;

const INVALID_SPAN: Range<usize> = Range {
    start: usize::MAX,
    end: usize::MAX,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match<'h> {
    hay: &'h str,
    spans: [Range<usize>; GROUP_COUNT],
}

impl<'h> ::recz::str::Capture<'h> for Match<'h> {
    #[inline]
    fn as_str(&self) -> &'h str {
        &self.hay[self.span()]
    }

    #[inline]
    fn start(&self) -> usize {
        self.spans[0].start
    }

    #[inline]
    fn end(&self) -> usize {
        self.spans[0].end
    }
}

impl<'h> ::recz::str::Match<'h> for Match<'h> {
    #[inline]
    fn haystack(&self) -> &'h str {
        self.hay
    }

    fn group<'a>(
        &self,
        label: impl Into<CaptureLabel<'a>>,
    ) -> Option<impl ::recz::str::Capture<'h>> {
        let index = match label.into() {
            CaptureLabel::Digit(0) => 0,
            CaptureLabel::Digit(1) => 1,
            CaptureLabel::Digit(2) => 2,
            _ => return None,
        };
        self.to_option(&self.spans[index])
    }

    fn groups(&self) -> impl ::core::iter::Iterator<Item = impl ::recz::str::Capture<'h>> {
        self.spans.iter().filter_map(|span| self.to_option(span))
    }
}

impl<'h> Match<'h> {
    #[inline]
    pub fn as_str(&self) -> &'h str {
        ::recz::str::Capture::as_str(self)
    }

    #[inline]
    pub fn span(&self) -> Range<usize> {
        ::recz::str::Capture::span(self)
    }

    #[inline]
    pub fn start(&self) -> usize {
        ::recz::str::Capture::start(self)
    }

    #[inline]
    pub fn end(&self) -> usize {
        ::recz::str::Capture::end(self)
    }

    #[inline]
    pub fn len(&self) -> usize {
        ::recz::str::Capture::len(self)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        ::recz::str::Capture::is_empty(self)
    }

    #[inline]
    pub fn haystack(&self) -> &'h str {
        ::recz::str::Match::haystack(self)
    }

    #[inline]
    pub fn group(&self, label: u32) -> Option<impl ::recz::str::Capture<'h>> {
        ::recz::str::Match::group(self, label)
    }

    #[inline]
    pub fn groups(&self) -> impl ::core::iter::Iterator<Item = impl ::recz::str::Capture<'h>> {
        ::recz::str::Match::groups(self)
    }
}

impl<'h> Match<'h> {
    fn to_option(&self, range: &Range<usize>) -> Option<Capture<'h>> {
        if range.end == usize::MAX {
            Some(Capture {
                capture: &self.hay[*range],
                start: range.start,
            })
        } else {
            None
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    fi_0,
    fi_1,
    fi_2,
    ge_3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Regex;

impl ::recz::str::Regex for Regex {
    #[inline]
    fn pattern(&self) -> &'static str {
        "(?<0>(?<2>[a-c])*(?<3>[a-f])*)"
    }

    #[inline(always)]
    fn find<'h>(&self, haystack: &'h str) -> Option<impl ::recz::str::Match<'h>> {
        let mut m: Option<Match<'h>> = None;
        self.find_impl(haystack, &mut m);
        m
    }
}

impl Regex {
    #[inline]
    pub fn pattern(&self) -> &'static str {
        ::recz::str::Regex::pattern(self)
    }

    #[inline(always)]
    pub fn find<'h>(&self, haystack: &'h str) -> Option<impl ::recz::str::Match<'h>> {
        ::recz::str::Regex::find(self, haystack)
    }

    fn find_impl<'h>(&self, haystack: &'h str, result: &mut Option<Match<'h>>) {
        let mut spans = [INVALID_SPAN; GROUP_COUNT];
        let mut pos = 0;
        let mut curr_state = State::fi_0;
        let mut iter = haystack.as_bytes().iter().copied();
        'main: loop {
            let byte = iter.next();
            match curr_state {
                State::fi_0 => {
                    match byte {
                        Some(b'a'..=b'c') => {
                            spans[0].start = pos;
                            spans[1].start = pos;
                            spans[2].start = pos;
                            curr_state = State::fi_1;
                        }
                        Some(b'd'..=b'f') => {
                            spans[0].start = pos;
                            spans[2].start = pos;
                            curr_state = State::fi_2;
                        }
                        _ => {
                            spans[0].start = pos;
                            spans[0].end = pos;
                            curr_state = State::ge_3;
                        }
                    };
                }
                State::fi_1 => {
                    match byte {
                        Some(b'a'..=b'c') => {
                            spans[1].start = pos;
                            spans[2].start = pos;
                            spans[1].end = pos;
                            spans[2].end = pos;
                            curr_state = State::fi_1;
                        }
                        Some(b'd'..=b'f') => {
                            spans[2].start = pos;
                            spans[1].end = pos;
                            spans[2].end = pos;
                            curr_state = State::fi_2;
                        }
                        _ => {
                            spans[0].end = pos;
                            spans[1].end = pos;
                            spans[2].end = pos;
                            curr_state = State::ge_3;
                        }
                    };
                }
                State::fi_2 => {
                    match byte {
                        Some(b'a'..=b'f') => {
                            spans[2].start = pos;
                            spans[2].end = pos;
                            curr_state = State::fi_2;
                        }
                        _ => {
                            spans[0].end = pos;
                            spans[2].end = pos;
                            curr_state = State::ge_3;
                        }
                    };
                }
                State::ge_3 => {
                    *result = Some(Match {
                        hay: haystack,
                        spans,
                    });
                    break 'main;
                }
            }
            pos += 1;
        }
    }
}

#[test]
fn real_test() {
    let re = Regex;
    let m = re.find("aaccff");
    assert!(m.is_some());
}
