use crate::Codec;
use crate::Encoding;
use crate::{Error::*, Result};
use recz_adt::Range;

const ENCODING: Encoding = Encoding::Latin1;

pub struct Latin1Codec;

impl Latin1Codec {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for Latin1Codec {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Codec for Latin1Codec {
    #[inline]
    fn encoding(&self) -> Encoding {
        ENCODING
    }

    fn encode_char(&self, c: char, buffer: &mut [u8]) -> Result<usize> {
        let c = ucp_to_latin1(c as u32)?;
        if buffer.is_empty() {
            Err(SmallBuffer)
        } else {
            buffer[0] = c;
            Ok(1)
        }
    }

    fn encode_ucp(&self, codepoint: u32, buffer: &mut [u8]) -> Result<usize> {
        let c = ucp_to_latin1(codepoint)?;
        if buffer.is_empty() {
            Err(SmallBuffer)
        } else {
            buffer[0] = c;
            Ok(1)
        }
    }

    fn encode_str(&self, s: &str, buffer: &mut [u8]) -> Result<usize> {
        let mut count = 0;
        let buf_len = buffer.len();
        for c in s.chars() {
            if count >= buf_len {
                return Err(SmallBuffer);
            }
            self.encode_char(c, &mut buffer[count..])?;
            count += 1;
        }
        Ok(count)
    }

    fn encode_range<F>(&self, start_ucp: u32, end_ucp: u32, handler: F) -> Result<()>
    where
        F: FnMut(&[Range<u8>]),
    {
        let start_c = ucp_to_latin1(start_ucp)?;
        let end_c = ucp_to_latin1(end_ucp)?;
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

    fn verify_codepoint(&self, codepoint: u32) -> Result<()> {
        ucp_to_latin1(codepoint).map(|_| ())
    }
}

fn ucp_to_latin1(codepoint: u32) -> Result<u8> {
    if codepoint <= ENCODING.max_codepoint() {
        Ok(codepoint as u8)
    } else {
        Err(InvalidCodePoint {
            codepoint,
            encoding: ENCODING,
        })
    }
}
