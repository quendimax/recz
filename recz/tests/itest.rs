use pretty_assertions::assert_eq;

#[test]
fn hello() {
    let re = recz::re!("h[ae]llo");
    assert_eq!(re.pattern(), "h[ae]llo");
}
