use pretty_assertions::assert_eq;
use recz::re;

#[test]
fn ab_star_a_star() {
    let re = re!("(?<ab>[ab]*)(?<a>a*)");

    let m = re.mtch("aaa").unwrap();
    assert_eq!(m.haystack(), "aaa");
    assert_eq!(m.capture(0).unwrap().range(), (0..3).into());
    assert_eq!(m.capture("ab").unwrap().range(), (0..3).into());
    assert_eq!(m.capture("a").unwrap().range(), (3..3).into());

    let m = re.mtch("babaa").unwrap();
    assert_eq!(m.haystack(), "babaa");
    assert_eq!(m.capture(0).unwrap().range(), (0..5).into());
    assert_eq!(m.capture("ab").unwrap().range(), (0..5).into());
    assert_eq!(m.capture("a").unwrap().range(), (5..5).into());
}

#[test]
fn a_star_star() {
    let re = re!("(?<sup>(?<sub>a*)*)");

    let m = re.mtch("aaa").unwrap();
    assert_eq!(m.haystack(), "aaa");
    assert_eq!(m.capture(0).unwrap().range(), (0..3).into());
    assert_eq!(m.capture("sup").unwrap().range(), (0..3).into());
    assert_eq!(m.capture("sub").unwrap().range(), (3..3).into());
}
