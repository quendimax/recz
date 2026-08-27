use recz_codec as codec;
use recz_graph::CaptureLabel;
use std::ops::Range;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Error, Debug, PartialEq)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum Error {
    #[error("codec error: {cause}")]
    CodecError {
        #[source]
        cause: codec::Error,

        #[cfg_attr(feature = "miette", label("invalid codepoint"))]
        span: Range<usize>,
    },

    #[error("expected {expected}, but found `{misspell}`")]
    Unexpected {
        misspell: Box<str>,
        expected: Box<str>,

        #[cfg_attr(feature = "miette", label("unexpected token"))]
        span: Range<usize>,
    },

    #[error("value `{value}` is out of {range}")]
    OutOfRange {
        value: Box<str>,
        range: Box<str>,

        #[cfg_attr(feature = "miette", label("invalid value"))]
        span: Range<usize>,
    },

    #[error("empty escape expression is not allowed")]
    EmptyEscape {
        #[cfg_attr(feature = "miette", label("empty escape"))]
        span: Range<usize>,
    },

    #[error("unsupported escape sequence `{sequence}`")]
    UnsupportedEscape {
        sequence: Box<str>,

        #[cfg_attr(feature = "miette", label("unsupported escape"))]
        span: Range<usize>,
    },

    #[error("zero repetition `{{0,0}}` is not allowed")]
    ZeroRepetition {
        #[cfg_attr(feature = "miette", label("zero repetition"))]
        span: Range<usize>,
    },

    #[error("repetition expression `{{n,m}}` expects that `n <= m`")]
    InvalidRepetition {
        #[cfg_attr(feature = "miette", label("invalid repetition"))]
        span: Range<usize>,
    },

    #[error("using capture label `{label}` more than once is not allowed")]
    CaptureLabelReuse {
        label: CaptureLabel,

        #[cfg_attr(feature = "miette", label("group name is already used"))]
        span: Range<usize>,
    },

    #[error(
        r"range `{range}` is inverted: first codepoint `\x{fcp:X}` is greater than last one `\x{lcp:X}`"
    )]
    #[cfg_attr(feature = "miette", diagnostic(help("try swapping the codepoints")))]
    InvertedRange {
        range: Box<str>,
        fcp: u32,
        lcp: u32,

        #[cfg_attr(feature = "miette", label("inverted range"))]
        span: Range<usize>,
    },

    #[error(r"capture label contains disallowed characters")]
    InvalidCaptureLabelChar {
        #[cfg_attr(feature = "miette", label("disallowed character"))]
        span: Range<usize>,
    },

    #[error(r"group with prefix `{prefix}` is not supported")]
    UnsupportedGroup {
        prefix: Box<str>,

        #[cfg_attr(feature = "miette", label("the prefix"))]
        span: Range<usize>,
    },

    #[error(r"capture group label must contain at least one character")]
    EmptyCaptureLabel {
        #[cfg_attr(feature = "miette", label("empty capture label"))]
        span: Range<usize>,
    },
}

/// Helper module to facilitate creating new error instances.
pub(crate) mod err {
    use super::*;

    pub(crate) fn codec_error<T>(cause: codec::Error, span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::CodecError { cause, span }))
    }

    pub(crate) fn unexpected<T>(
        misspell: impl Into<Box<str>>,
        misspan: Range<usize>,
        expected: impl Into<Box<str>>,
    ) -> Result<T> {
        Err(Box::new(Error::Unexpected {
            misspell: misspell.into(),
            span: misspan,
            expected: expected.into(),
        }))
    }

    pub(crate) fn out_of_range<T>(
        value: impl Into<Box<str>>,
        span: Range<usize>,
        range: impl Into<Box<str>>,
    ) -> Result<T> {
        Err(Box::new(Error::OutOfRange {
            value: value.into(),
            span,
            range: range.into(),
        }))
    }

    pub(crate) fn empty_escape<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::EmptyEscape { span }))
    }

    pub(crate) fn unsupported_escape<T, S>(sequence: S, span: Range<usize>) -> Result<T>
    where
        S: Into<Box<str>>,
    {
        Err(Box::new(Error::UnsupportedEscape {
            sequence: sequence.into(),
            span,
        }))
    }

    pub(crate) fn zero_repetition<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::ZeroRepetition { span }))
    }

    pub(crate) fn invalid_repetition<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::InvalidRepetition { span }))
    }

    pub(crate) fn reuse_capture_label<T>(
        label: impl Into<CaptureLabel>,
        span: Range<usize>,
    ) -> Result<T> {
        Err(Box::new(Error::CaptureLabelReuse {
            label: label.into(),
            span,
        }))
    }

    pub(crate) fn inverted_range<T>(
        range: impl Into<Box<str>>,
        first_codepoint: u32,
        last_codepoint: u32,
        span: Range<usize>,
    ) -> Result<T> {
        Err(Box::new(Error::InvertedRange {
            range: range.into(),
            fcp: first_codepoint,
            lcp: last_codepoint,
            span,
        }))
    }

    pub(crate) fn invalid_capture_label_char<T>(disallows_char_span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::InvalidCaptureLabelChar {
            span: disallows_char_span,
        }))
    }

    pub(crate) fn unsupported_group<T>(
        prefix: impl Into<Box<str>>,
        span: Range<usize>,
    ) -> Result<T> {
        Err(Box::new(Error::UnsupportedGroup {
            prefix: prefix.into(),
            span,
        }))
    }

    pub(crate) fn empty_capture_label<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::EmptyCaptureLabel { span }))
    }
}
