use crate::encoding::Encoding;
use crate::error::Result;
use recz_adt::Range;

/// This trait helps convert unicode code points into byte sequences
/// corresponding to the encoding way chosen by the user.
pub trait Codec {
    /// Returns the [`Encoding`] enum that represents the base information about
    /// the encoding system.
    fn encoding(&self) -> Encoding;

    /// Encodes unicode code point into a byte sequence
    fn encode_ucp(&self, codepoint: u32, buffer: &mut [u8]) -> Result<usize>;

    /// Encodes char into a byte sequence.
    fn encode_char(&self, c: char, buffer: &mut [u8]) -> Result<usize>;

    /// Encodes string into a byte sequence.
    fn encode_str(&self, s: &str, buffer: &mut [u8]) -> Result<usize>;

    /// Encodes range of unicode code points into array of byte sequences.
    ///
    /// If input range contains invalid code points, the method should ignore
    /// them.
    fn encode_range<F>(&self, start_ucp: u32, end_ucp: u32, handler: F) -> Result<()>
    where
        F: FnMut(&[Range<u8>]);

    /// Encodes the entire range of code points allowed by this coder into array
    /// of byte sequences.
    fn encode_entire_range<F>(&self, handler: F)
    where
        F: FnMut(&[Range<u8>]);

    /// Verifies if the given code point is valid for this codec. If not,
    /// returns an error with description what is wrong with the codepoint.
    fn verify_codepoint(&self, codepoint: u32) -> Result<()>;
}
