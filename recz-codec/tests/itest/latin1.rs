use pretty_assertions::assert_eq;
use recz_adt::Range;
use recz_codec::{Codec, Error, Latin1Codec};
use std::assert_matches;

static CODEC: Latin1Codec = Latin1Codec;

#[test]
fn encoding() {
    #[allow(clippy::default_constructed_unit_structs)]
    let codec = Latin1Codec::default();
    let encoding = codec.encoding();
    assert_eq!(encoding.name(), "Latin-1");
}

#[test]
fn encode_char() {
    let mut buffer = [0u8; 1];
    let result = CODEC.encode_char('a', &mut buffer);
    assert_eq!(result, Ok(1));
    assert_eq!(buffer[0], 0x61);
}

#[test]
fn encode_char_fails() {
    let mut buffer = [0u8; 1];
    let result = CODEC.encode_char('€', &mut buffer);
    assert_eq!(
        result,
        Err(Error::InvalidCodePoint {
            codepoint: '€' as u32,
            encoding: CODEC.encoding()
        })
    );

    let mut buffer = [0u8; 0];
    let result = CODEC.encode_char('a', &mut buffer);
    assert_eq!(result, Err(Error::SmallBuffer));
}

#[test]
fn encode_ucp() {
    let mut buffer = [0u8; 1];
    assert_eq!(CODEC.encode_ucp('a' as u32, &mut buffer), Ok(1));
    assert_eq!(buffer, [b'a']);
}

#[test]
fn encode_ucp_fails() {
    let mut buffer = [0u8; 0];
    assert_eq!(
        CODEC.encode_ucp('a' as u32, &mut buffer),
        Err(Error::SmallBuffer)
    );

    let mut buffer = [0u8; 1];
    assert_matches!(
        CODEC.encode_ucp(256, &mut buffer),
        Err(Error::InvalidCodePoint { codepoint: 256, .. })
    );
}

#[test]
fn encode_str() {
    let mut buffer = [0u8; 9];
    assert_eq!(CODEC.encode_str("abcó", &mut buffer), Ok(4));
    assert_eq!(&buffer[..4], [b'a', b'b', b'c', b'\xF3']);
}

#[test]
fn encode_str_fails() {
    let mut buffer = [0u8; 2];
    assert_eq!(
        CODEC.encode_str("abc", &mut buffer),
        Err(Error::SmallBuffer)
    );

    let mut buffer = [0u8; 1];
    assert_matches!(
        CODEC.encode_str("\u{100}", &mut buffer),
        Err(Error::InvalidCodePoint { codepoint: 256, .. })
    );
}

#[test]
fn encode_range() {
    assert!(
        CODEC
            .encode_range('a' as u32, 'z' as u32, |ranges| {
                assert_eq!(ranges, &[Range::new(b'a', b'z')])
            })
            .is_ok()
    );
    assert!(
        CODEC
            .encode_range('z' as u32, 'a' as u32, |ranges| {
                assert_eq!(ranges, &[Range::new(b'a', b'z')])
            })
            .is_ok()
    );
}

#[test]
fn encode_range_fails() {
    assert_matches!(
        CODEC.encode_range('ў' as u32, 'z' as u32, |_| {}),
        Err(Error::InvalidCodePoint {
            codepoint: 1118,
            ..
        })
    );
    assert_matches!(
        CODEC.encode_range('a' as u32, 'ў' as u32, |_| {}),
        Err(Error::InvalidCodePoint {
            codepoint: 1118,
            ..
        })
    );
}

#[test]
fn encode_entire_range() {
    CODEC.encode_entire_range(|ranges| assert_eq!(ranges, &[Range::new(0, 255)]));
}

#[test]
fn verify_codepoint() {
    assert!(CODEC.verify_codepoint(0).is_ok());
    assert!(CODEC.verify_codepoint(255).is_ok());
    assert_matches!(
        CODEC.verify_codepoint(256),
        Err(Error::InvalidCodePoint { codepoint: 256, .. })
    );
}
