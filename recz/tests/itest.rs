use pretty_assertions::assert_eq;
use recz::Label;

#[rust_analyzer::skip]
#[test]
fn hello() {
    let re = recz::re!("h[ae](?<tail>llo)");
    assert_eq!(re.pattern(), "h[ae](?<tail>llo)");
    assert_eq!(re.capture_labels(), [Label::Num(0), Label::Str("tail")]);

    let _ = re.mtch("hello");
}
