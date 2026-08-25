use miette::{Diagnostic, SourceSpan};
use recz_codec as codec;
use recz_graph::CaptureLabel;
use std::ops::Range;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum Error {
    #[error("codec error: {cause}")]
    CodecError {
        #[source]
        cause: codec::Error,

        #[label("invalid codepoint")]
        span: SourceSpan,
    },

    #[error("expected {expected}, but found `{misspell}`")]
    Unexpected {
        misspell: Box<str>,
        expected: Box<str>,

        #[label("unexpected token")]
        span: SourceSpan,
    },

    #[error("value `{value}` is out of {range}")]
    OutOfRange {
        value: Box<str>,
        range: Box<str>,

        #[label("invalid value")]
        span: SourceSpan,
    },

    #[error("empty escape expression is not allowed")]
    EmptyEscape {
        #[label("empty escape")]
        span: SourceSpan,
    },

    #[error("unsupported escape sequence `{sequence}`")]
    UnsupportedEscape {
        sequence: Box<str>,

        #[label("unsupported escape")]
        span: SourceSpan,
    },

    #[error("zero repetition `{{0,0}}` is not allowed")]
    ZeroRepetition {
        #[label("zero repetition")]
        span: SourceSpan,
    },

    #[error("repetition expression `{{n,m}}` expects that `n <= m`")]
    InvalidRepetition {
        #[label("invalid repetition")]
        span: SourceSpan,
    },

    #[error("using capture label `{label}` more than once is not allowed")]
    CaptureLabelReuse {
        label: CaptureLabel,

        #[label("group name is already used")]
        span: SourceSpan,
    },

    #[error(
        r"range `[{range}]` is inverted: first codepoint `\x{fcp:X}` is greater than last one `\x{lcp:X}`"
    )]
    #[diagnostic(help("try swapping the codepoints"))]
    InvertedRange {
        range: Box<str>,
        fcp: u32,
        lcp: u32,

        #[label("inverted range")]
        span: SourceSpan,
    },

    #[error(r"capture label contains disallowed characters")]
    InvalidCaptureLabelChar {
        #[label("disallowed character")]
        span: SourceSpan,
    },

    #[error(r"group with prefix `{prefix}` is not supported")]
    UnsupportedGroup {
        prefix: Box<str>,

        #[label("the prefix")]
        span: SourceSpan,
    },

    #[error(r"capture group label must contain at least one character")]
    EmptyCaptureLabel {
        #[label("empty capture label")]
        span: SourceSpan,
    },
}

/// Helper module to facilitate creating new error instances.
pub(crate) mod err {
    use super::*;

    pub(crate) fn codec_error<T>(cause: codec::Error, span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::CodecError {
            cause,
            span: span.into(),
        }))
    }

    pub(crate) fn unexpected<T>(
        misspell: impl Into<Box<str>>,
        misspan: Range<usize>,
        expected: impl Into<Box<str>>,
    ) -> Result<T> {
        Err(Box::new(Error::Unexpected {
            misspell: misspell.into(),
            span: misspan.into(),
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
            span: span.into(),
            range: range.into(),
        }))
    }

    pub(crate) fn empty_escape<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::EmptyEscape { span: span.into() }))
    }

    pub(crate) fn unsupported_escape<T, S>(sequence: S, span: Range<usize>) -> Result<T>
    where
        S: Into<Box<str>>,
    {
        Err(Box::new(Error::UnsupportedEscape {
            sequence: sequence.into(),
            span: span.into(),
        }))
    }

    pub(crate) fn zero_repetition<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::ZeroRepetition { span: span.into() }))
    }

    pub(crate) fn invalid_repetition<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::InvalidRepetition { span: span.into() }))
    }

    pub(crate) fn reuse_capture_label<T>(
        label: impl Into<CaptureLabel>,
        span: Range<usize>,
    ) -> Result<T> {
        Err(Box::new(Error::CaptureLabelReuse {
            label: label.into(),
            span: span.into(),
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
            span: span.into(),
        }))
    }

    pub(crate) fn invalid_capture_label_char<T>(disallows_char_span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::InvalidCaptureLabelChar {
            span: disallows_char_span.into(),
        }))
    }

    pub(crate) fn unsupported_group<T>(
        prefix: impl Into<Box<str>>,
        span: Range<usize>,
    ) -> Result<T> {
        Err(Box::new(Error::UnsupportedGroup {
            prefix: prefix.into(),
            span: span.into(),
        }))
    }

    pub(crate) fn empty_capture_label<T>(span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::EmptyCaptureLabel { span: span.into() }))
    }
}
