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

#[test]
fn class_plus() {
    let re = re!("[a-z]+");
    assert_eq!(re.pattern(), "[a-z]+");
    assert_eq!(re.capture_labels(), [Label::Num(0)]);

    let m = re.mtch("hello123").unwrap();
    assert_eq!(m.as_str(), "hello");
    assert_eq!(m.range(), (0..5).into());

    assert_eq!(re.mtch("123"), None);
}

#[test]
fn negated_class() {
    let re = re!("[^0-9]+");

    let m = re.mtch("abc123").unwrap();
    assert_eq!(m.as_str(), "abc");
    assert_eq!(m.range(), (0..3).into());

    assert_eq!(re.mtch("123abc"), None);
}

#[test]
fn dot_wildcard() {
    let re = re!("h.llo");

    assert_eq!(re.mtch("hallo").unwrap().as_str(), "hallo");
    assert_eq!(re.mtch("hxllo").unwrap().as_str(), "hxllo");
    assert_eq!(re.mtch("hllo"), None);
}

#[test]
fn optional() {
    let re = re!("colou?r");
    assert_eq!(re.pattern(), "colou?r");

    assert_eq!(re.mtch("color").unwrap().as_str(), "color");
    assert_eq!(re.mtch("colour").unwrap().as_str(), "colour");
    assert_eq!(re.mtch("colouur"), None);
}

#[test]
fn disjunction() {
    let re = re!("cat|dog");

    let m = re.mtch("cats").unwrap();
    assert_eq!(m.as_str(), "cat");

    let m = re.mtch("dogs").unwrap();
    assert_eq!(m.as_str(), "dog");

    assert_eq!(re.mtch("bird"), None);
}

#[test]
fn non_capturing_group() {
    let re = re!("(?:ab)+c");
    assert_eq!(re.capture_labels(), [Label::Num(0)]);

    assert_eq!(re.mtch("ababc").unwrap().as_str(), "ababc");
    assert_eq!(re.mtch("abc").unwrap().as_str(), "abc");
    assert_eq!(re.mtch("c"), None);
}

#[test]
fn exact_repeat() {
    let re = re!("a{3}");

    let m = re.mtch("aaaa").unwrap();
    assert_eq!(m.as_str(), "aaa");
    assert_eq!(m.range(), (0..3).into());

    assert_eq!(re.mtch("aa"), None);
}

#[test]
fn range_repeat() {
    let re = re!("a{2,4}");

    let m = re.mtch("aaaaaa").unwrap();
    assert_eq!(m.as_str(), "aaaa");

    assert_eq!(re.mtch("a"), None);
}

#[test]
fn min_repeat() {
    let re = re!("a{2,}");

    let m = re.mtch("aaaaa").unwrap();
    assert_eq!(m.as_str(), "aaaaa");

    assert_eq!(re.mtch("a"), None);
}

#[test]
fn numbered_groups() {
    let re = re!("(?D<1>[a-z]+)@(?D<2>[a-z]+)");
    assert_eq!(
        re.capture_labels(),
        [Label::Num(0), Label::Num(1), Label::Num(2)]
    );

    let m = re.mtch("user@example.com").unwrap();
    assert_eq!(m.as_str(), "user@example");
    assert_eq!(m.capture(0).unwrap().as_str(), "user@example");
    assert_eq!(m.capture(1).unwrap().as_str(), "user");
    assert_eq!(m.capture(2).unwrap().as_str(), "example");

    assert_eq!(re.mtch("@example.com"), None);
}

#[test]
fn nested_groups() {
    let re = re!("(?:a(?D<1>b|c)d)+");
    assert_eq!(re.capture_labels(), [Label::Num(0), Label::Num(1)]);

    let m = re.mtch("abdacd").unwrap();
    assert_eq!(m.as_str(), "abdacd");
    assert_eq!(m.capture(1).unwrap().as_str(), "c");
}
