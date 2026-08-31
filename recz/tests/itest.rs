use pretty_assertions::assert_eq;
use recz::Label;

#[test]
fn hello() {
    let re = recz::re!("h[ae](?<tail>llo)");
    assert_eq!(re.pattern(), "h[ae](?<tail>llo)");
    assert_eq!(re.capture_labels(), [Label::Num(0), Label::Str("tail")]);

    let m = re.mtch("hello").unwrap();
    assert_eq!(m.haystack(), "hello");
    assert_eq!(m.capture(0), None);
}
