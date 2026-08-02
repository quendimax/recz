use miette::{Diagnostic, SourceSpan};
use recz_codec as codec;
use std::ops::Range;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum Error {
    #[error("encoder error: {cause}")]
    EncoderError {
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

    #[error("reuse group name `{group_label}` more than once is not allowed")]
    GroupNameReuse {
        group_label: u32,

        #[label("group name is already used")]
        span: SourceSpan,
    },
}

/// Helper module to facilitate creating new error instances.
pub(crate) mod err {
    use super::*;

    pub(crate) fn encoder_error<T>(cause: codec::Error, span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::EncoderError {
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

    pub(crate) fn reuse_group_name<T>(group_label: u32, span: Range<usize>) -> Result<T> {
        Err(Box::new(Error::GroupNameReuse {
            group_label,
            span: span.into(),
        }))
    }
}
