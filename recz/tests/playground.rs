use ::core::iter::Iterator;
use ::core::range::Range;

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
    pub fn range(&self) -> Range<usize> {
        ::recz::str::Capture::range(self)
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

const INVALID_RANGE: Range<usize> = Range {
    start: usize::MAX,
    end: usize::MAX,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match<'h> {
    hay: &'h str,
    ranges: [Range<usize>; GROUP_COUNT],
}

impl<'h> ::recz::str::Capture<'h> for Match<'h> {
    #[inline]
    fn as_str(&self) -> &'h str {
        &self.hay[self.range()]
    }

    #[inline]
    fn start(&self) -> usize {
        self.ranges[0].start
    }

    #[inline]
    fn end(&self) -> usize {
        self.ranges[0].end
    }
}

impl<'h> ::recz::str::Match<'h> for Match<'h> {
    #[inline]
    fn haystack(&self) -> &'h str {
        self.hay
    }

    fn group_by_num(&self, num: u32) -> Option<impl ::recz::str::Capture<'h>> {
        let index = match num {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => return None,
        };
        self.to_option(&self.ranges[index])
    }

    fn group_by_str<'a>(&self, label: &'a str) -> Option<impl ::recz::str::Capture<'h>> {
        let _ = label;
        Option::<Capture<'h>>::None
    }

    fn groups(&self) -> impl ::core::iter::Iterator<Item = impl ::recz::str::Capture<'h>> {
        self.ranges.iter().filter_map(|range| self.to_option(range))
    }
}

impl<'h> Match<'h> {
    #[inline]
    pub fn as_str(&self) -> &'h str {
        ::recz::str::Capture::as_str(self)
    }

    #[inline]
    pub fn range(&self) -> Range<usize> {
        ::recz::str::Capture::range(self)
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
    pub fn group_by_num(&self, label: u32) -> Option<impl ::recz::str::Capture<'h>> {
        ::recz::str::Match::group_by_num(self, label)
    }

    #[inline]
    pub fn group_by_str<'a>(&self, label: &'a str) -> Option<impl ::recz::str::Capture<'h>> {
        ::recz::str::Match::group_by_str(self, label)
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
        let mut ranges = [INVALID_RANGE; GROUP_COUNT];
        let mut pos = 0;
        let mut curr_state = State::fi_0;
        let mut iter = haystack.as_bytes().iter().copied();
        'main: loop {
            let byte = iter.next();
            match curr_state {
                State::fi_0 => {
                    match byte {
                        Some(b'a'..=b'c') => {
                            ranges[0].start = pos;
                            ranges[1].start = pos;
                            ranges[2].start = pos;
                            curr_state = State::fi_1;
                        }
                        Some(b'd'..=b'f') => {
                            ranges[0].start = pos;
                            ranges[2].start = pos;
                            curr_state = State::fi_2;
                        }
                        _ => {
                            ranges[0].start = pos;
                            ranges[0].end = pos;
                            curr_state = State::ge_3;
                        }
                    };
                }
                State::fi_1 => {
                    match byte {
                        Some(b'a'..=b'c') => {
                            ranges[1].start = pos;
                            ranges[2].start = pos;
                            ranges[1].end = pos;
                            ranges[2].end = pos;
                            curr_state = State::fi_1;
                        }
                        Some(b'd'..=b'f') => {
                            ranges[2].start = pos;
                            ranges[1].end = pos;
                            ranges[2].end = pos;
                            curr_state = State::fi_2;
                        }
                        _ => {
                            ranges[0].end = pos;
                            ranges[1].end = pos;
                            ranges[2].end = pos;
                            curr_state = State::ge_3;
                        }
                    };
                }
                State::fi_2 => {
                    match byte {
                        Some(b'a'..=b'f') => {
                            ranges[2].start = pos;
                            ranges[2].end = pos;
                            curr_state = State::fi_2;
                        }
                        _ => {
                            ranges[0].end = pos;
                            ranges[2].end = pos;
                            curr_state = State::ge_3;
                        }
                    };
                }
                State::ge_3 => {
                    *result = Some(Match {
                        hay: haystack,
                        ranges,
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
    let pat = re.pattern();
    assert!(pat.contains("a"))
}
