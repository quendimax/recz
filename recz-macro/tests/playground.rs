use ::core::iter::Iterator;
use ::core::range::Range;

pub mod api {
    use ::core::range::Range;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Capture<'h> {
        capture: &'h str,
        start: usize,
    }

    impl<'h> Capture<'h> {
        #[inline]
        pub fn __new(capture: &'h str, start: usize) -> Self {
            Self { capture, start }
        }

        #[inline]
        pub fn start(&self) -> usize {
            self.start
        }

        #[inline]
        pub fn end(&self) -> usize {
            self.start + self.capture.len()
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.capture.len()
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.capture.is_empty()
        }

        #[inline]
        pub fn span(&self) -> Range<usize> {
            Range {
                start: self.start(),
                end: self.end(),
            }
        }

        #[inline]
        pub fn as_str(&self) -> &'h str {
            self.capture
        }
    }
}

use api::Capture;

const GROUP_COUNT: usize = 3;

const INVALID_SPAN: Range<usize> = Range {
    start: usize::MAX,
    end: usize::MAX,
};

fn is_invalid(span: &Range<usize>) -> bool {
    span.end == usize::MAX
}

fn to_option<'h>(hay: &'h str, range: &Range<usize>) -> Option<Capture<'h>> {
    if !is_invalid(range) {
        Some(Capture::__new(&hay[*range], range.start))
    } else {
        None
    }
}

pub struct Match<'h> {
    hay: &'h str,
    spans: [Range<usize>; GROUP_COUNT],
}

impl<'h> Match<'h> {
    fn new(hay: &'h str) -> Self {
        Self {
            hay,
            spans: [INVALID_SPAN; GROUP_COUNT],
        }
    }

    #[inline]
    pub fn haystack(&self) -> &'h str {
        self.hay
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.spans[0].start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.spans[0].end
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end() - self.start()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn span(&self) -> Range<usize> {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'h str {
        &self.hay[self.span()]
    }

    pub fn group(&self, label: u32) -> Option<Capture<'h>> {
        let index = match label {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => return None,
        };
        to_option(self.hay, &self.spans[index])
    }

    pub fn group_str(&self, label: &str) -> Option<Capture<'h>> {
        let index = match label {
            "0" => 0,
            "hello" => 0,
            "bye" => 2,
            _ => return None,
        };
        to_option(self.hay, &self.spans[index])
    }

    pub fn groups(&self) -> impl Iterator<Item = Capture<'h>> {
        self.spans
            .iter()
            .filter_map(|span| to_option(self.hay, span))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    fi_0,
    fi_1,
    fi_2,
    ge_3,
}

pub struct Regex;

impl Regex {
    pub fn pattern(&self) -> &str {
        "(?<0>(?<2>[a-c])*(?<3>[a-f])*)"
    }

    #[inline(always)]
    pub fn test<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {
        let mut m: Option<Match<'h>> = None;
        self.test_impl(haystack, &mut m);
        m
    }

    fn test_impl<'h>(&self, haystack: &'h str, result: &mut Option<Match<'h>>) {
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
                    *result = Some(Match {
                        hay: haystack,
                        spans,
                    });
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
                    *result = Some(Match {
                        hay: haystack,
                        spans,
                    });
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
                    *result = Some(Match {
                        hay: haystack,
                        spans,
                    });
                }
                State::ge_3 => break 'main,
            }
            pos += 1;
        }
    }
}

#[inline(never)]
pub fn test_regex() {
    let mut m = Match::new("hello");
    m.spans[0].start = 0;
    m.spans[0].end = 4;
    assert!(m.group_str("hello").is_some());
    let capt = m.group_str("hello").unwrap();
    assert_eq!(capt.as_str(), "hell");
}

#[test]
fn real_test() {
    test_regex();
}
