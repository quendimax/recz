use ::core::range::Range;
use ::recz::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capture<'h> {
    capture: &'h str,
    start: usize,
}

impl<'h> Capture<'h> {
    #[inline]
    pub fn as_str(&self) -> &'h str {
        self.capture
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
    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.capture.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.capture.is_empty()
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

impl<'h> Match<'h> {
    #[inline]
    pub fn as_str(&self) -> &'h str {
        &self.hay[self.range()]
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.ranges[0].start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.ranges[0].end
    }

    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.ranges[0]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ranges[0].end - self.ranges[0].start
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ranges[0].end == self.ranges[0].start
    }

    #[inline]
    pub fn haystack(&self) -> &'h str {
        self.hay
    }

    #[inline(always)]
    pub fn capture<'a>(&self, label: impl Into<Label<'a>>) -> Option<Capture<'h>> {
        match label.into() {
            Label::Num(num) => self.capture_by_num(num),
            Label::Str(name) => self.capture_by_str(name),
        }
    }

    pub fn capture_by_num(&self, number: u32) -> Option<Capture<'h>> {
        let index = match number {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => return None,
        };
        self.to_option(&self.ranges[index])
    }

    pub fn capture_by_str(&self, _name: &str) -> Option<Capture<'h>> {
        None
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

impl Regex {
    #[inline]
    pub fn pattern(&self) -> &'static str {
        "(?<0>(?<2>[a-c])*(?<3>[a-f])*)"
    }

    pub fn capture_names(&self) -> &'static [Label<'static>] {
        &[Label::Str("0"), Label::Str("2"), Label::Str("3")]
    }

    #[inline(always)]
    pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {
        let mut m: Option<Match<'h>> = None;
        self.find_impl(haystack, &mut m);
        m
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
