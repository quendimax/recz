use recz_macro::__re_impl as re;

#[test]
fn test_re() {
    let regex = re!("he");
    assert!(regex.test("hello").is_none());
}
