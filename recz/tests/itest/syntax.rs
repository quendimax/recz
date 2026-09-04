use pretty_assertions::assert_eq;
use recz::{Label, re};

#[test]
fn concat() {
    let re = re!("асюсяй");
    assert_eq!(re.pattern(), "асюсяй");
    assert_eq!(re.capture_labels(), [Label::Num(0)]);

    let m = re.mtch("асюсяйка").unwrap();
    assert_eq!(m.haystack(), "асюсяйка");
    assert_eq!(m.as_str(), "асюсяй");
    assert_eq!(m.range(), (0..12).into());
    assert_eq!(m.capture(0).unwrap().as_str(), "асюсяй");

    assert_eq!(re.mtch("асюсяюшка"), None);
}

#[test]
fn hello() {
    let re = re!("h[ae](?<tail>llo*)");
    assert_eq!(re.pattern(), "h[ae](?<tail>llo*)");
    assert_eq!(re.capture_labels(), [Label::Num(0), Label::Str("tail")]);

    let m = re.mtch("hello").unwrap();
    assert_eq!(m.haystack(), "hello");
    assert_eq!(m.capture(0).unwrap().as_str(), "hello");
    assert_eq!(m.capture("tail").unwrap().as_str(), "llo");

    let m = re.mtch("helloooooasdf").unwrap();
    assert_eq!(m.haystack(), "helloooooasdf");
    assert_eq!(m.capture(0).unwrap().as_str(), "hellooooo");
    assert_eq!(m.capture("tail").unwrap().as_str(), "llooooo");
}

#[test]
fn hello2() {
    let re = re!("h((?<left>a)|(?<right>e))(?<tail>llo*)");
    assert_eq!(re.pattern(), "h((?<left>a)|(?<right>e))(?<tail>llo*)");
    assert_eq!(
        re.capture_labels(),
        [
            Label::Num(0),
            Label::Str("left"),
            Label::Str("right"),
            Label::Str("tail")
        ]
    );

    let m = re.mtch("hello").unwrap();
    assert_eq!(m.haystack(), "hello");
    assert_eq!(m.capture(0).unwrap().as_str(), "hello");
    assert_eq!(m.capture("tail").unwrap().as_str(), "llo");
    assert_eq!(m.capture("left"), None);
    assert_eq!(m.capture("right").unwrap().as_str(), "e");

    let m = re.mtch("halloooooasdf").unwrap();
    assert_eq!(m.haystack(), "halloooooasdf");
    assert_eq!(m.capture(0).unwrap().as_str(), "hallooooo");
    assert_eq!(m.capture("tail").unwrap().as_str(), "llooooo");
    assert_eq!(m.capture("left").unwrap().as_str(), "a");
    assert_eq!(m.capture("right"), None);
}
