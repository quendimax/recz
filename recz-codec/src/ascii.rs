use crate::Codec;
use crate::Encoding;
use crate::{Error::*, Result};
use recz_adt::Range;

const ENCODING: Encoding = Encoding::Ascii;

pub struct AsciiCodec;

impl AsciiCodec {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for AsciiCodec {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Codec for AsciiCodec {
    #[inline]
    fn encoding(&self) -> Encoding {
        ENCODING
    }

    fn encode_char(&self, c: char, buffer: &mut [u8]) -> Result<usize> {
        if let Ok(c) = c.try_into() {
            if buffer.is_empty() {
                Err(SmallBuffer)
            } else {
                buffer[0] = c;
                Ok(1)
            }
        } else {
            Err(InvalidCodePoint {
                codepoint: c as u32,
                encoding: ENCODING,
            })
        }
    }

    fn encode_ucp(&self, codepoint: u32, buffer: &mut [u8]) -> Result<usize> {
        if let Ok(c) = codepoint.try_into() {
            if buffer.is_empty() {
                Err(SmallBuffer)
            } else {
                buffer[0] = c;
                Ok(1)
            }
        } else {
            Err(InvalidCodePoint {
                codepoint,
                encoding: ENCODING,
            })
        }
    }

    fn encode_str(&self, s: &str, buffer: &mut [u8]) -> Result<usize> {
        let mut count = 0;
        for c in s.chars() {
            if let Some(slice) = buffer.get_mut(count..) {
                self.encode_char(c, slice)?;
                count += 1;
            } else {
                return Err(SmallBuffer);
            }
        }
        Ok(count)
    }

    fn encode_range<F>(&self, start_ucp: u32, end_ucp: u32, handler: F) -> Result<()>
    where
        F: FnMut(&[Range<u8>]),
    {
        let start_c = ucp_to_ascii(start_ucp)?;
        let end_c = ucp_to_ascii(end_ucp)?;
        let mut handler = handler;
        handler(&[Range::new(start_c, end_c)]);
        Ok(())
    }

    fn encode_entire_range<F>(&self, handler: F)
    where
        F: FnMut(&[Range<u8>]),
    {
        let mut handler = handler;
        handler(&[Range::new(
            ENCODING.min_codepoint() as u8,
            ENCODING.max_codepoint() as u8,
        )]);
    }
}

fn ucp_to_ascii(codepoint: u32) -> Result<u8> {
    if codepoint <= ENCODING.max_codepoint() {
        Ok(codepoint as u8)
    } else {
        Err(InvalidCodePoint {
            codepoint,
            encoding: ENCODING,
        })
    }
}
