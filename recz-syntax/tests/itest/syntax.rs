use pretty_assertions::assert_eq;
use recz_codec::Utf8Codec;
use recz_syntax::Parser;

fn parse(pattern: &str) -> String {
    let parser = Parser::new(Utf8Codec::new());
    match parser.parse(pattern) {
        Ok(hir) => hir.to_string(),
        Err(err) => err.to_string(),
    }
}

#[test]
fn parse_regular() {
    assert_eq!(parse("asdf|dfgh"), r#""asdf" | "dfgh""#);
    assert_eq!(parse("(asdf)|(?<1>dfgh)"), r#""asdf" | (?<1> "dfgh" )"#);
    assert_eq!(parse("[sdf]"), r#"['d'] | ['f'] | ['s']"#);

    assert_eq!(parse("asd\\f"), "unsupported escape sequence `\\f`");
    assert_eq!(parse("(abc))"), "expected `EOF`, but found `)`");
    assert_eq!(parse("[asd\\f]"), "unsupported escape sequence `\\f`");
    assert_eq!(
        parse(r"\u{D800}"),
        "codec error: surrogate code point U+D800 is not supported by UTF-8"
    );
    assert_eq!(
        parse(r"\u{110000}"),
        "codec error: invalid unicode code point U+110000 for UTF-8 encoding"
    );
}

#[test]
fn parse_inverted_range() {
    assert_eq!(parse("[a-z]"), r#"['a'-'z']"#);
    assert_eq!(
        parse("[z-a]"),
        r#"range `[z-a]` is inverted: first codepoint `\x7A` is greater than last one `\x61`"#
    );
}
