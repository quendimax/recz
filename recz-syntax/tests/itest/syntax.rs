use pretty_assertions::assert_eq;
use recz_codec::Utf8Codec;
use recz_syntax::Parser;

#[test]
fn parser_parse() {
    let parse = |pattern: &str| {
        let parser = Parser::new(Utf8Codec::new());
        match parser.parse(pattern) {
            Ok(hir) => hir.to_string(),
            Err(err) => err.to_string(),
        }
    };

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
