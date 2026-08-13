use ::core::range::Range;

pub struct Capture<'h> {
    label: u32,
    hay: &'h str,
    span: Range<usize>,
}

impl<'h> Capture<'h> {
    #[inline]
    pub fn label(&self) -> u32 {
        self.label
    }

    #[inline]
    pub fn hay(&self) -> &'h str {
        self.hay
    }

    #[inline]
    pub fn capture(&self) -> &'h str {
        &self.hay[self.span]
    }

    #[inline]
    pub fn span(&self) -> Range<usize> {
        self.span
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.span.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.span.end
    }
}

pub struct Match<'h> {
    hay: &'h str,
    groups: [Range<usize>; 2],
}

impl<'h> Match<'h> {
    pub fn hay(&self) -> &'h str {
        self.hay
    }

    pub fn group(&self, label: u32) -> Option<Capture<'h>> {
        let index = match label {
            0 => Some(0),
            1 => Some(1),
            _ => None,
        };
        index.map(|idx| Capture {
            label,
            hay: self.hay,
            span: self.groups[idx],
        })
    }
}

pub mod api {
    pub trait Hello {
        fn hello(&self) -> bool;
    }
}
