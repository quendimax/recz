use recz_macro::re;

#[test]
fn test_re() {
    let s = re!("he");
    assert_eq!(s, "he");
}
