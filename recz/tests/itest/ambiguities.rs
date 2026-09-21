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
    assert_eq!(m.capture("ab").unwrap().as_str(), "babaa");
    assert_eq!(m.capture("a").unwrap().range(), (5..5).into());
    assert_eq!(m.capture("a").unwrap().as_str(), "");
}

#[test]
fn ab_star_ac_star() {
    let re = re!("(?<ab>[ab]*)(?<ac>[ac]*)");

    let m = re.mtch("aaa").unwrap();
    assert_eq!(m.capture(0).unwrap().range(), (0..3).into());
    assert_eq!(m.capture("ab").unwrap().range(), (0..3).into());
    assert_eq!(m.capture("ac").unwrap().range(), (3..3).into());

    let m = re.mtch("babaacac").unwrap();
    assert_eq!(m.capture(0).unwrap().range(), (0..8).into());
    assert_eq!(m.capture("ab").unwrap().range(), (0..5).into());
    assert_eq!(m.capture("ab").unwrap().as_str(), "babaa");
    assert_eq!(m.capture("ac").unwrap().range(), (5..8).into());
    assert_eq!(m.capture("ac").unwrap().as_str(), "cac");
}

#[test]
#[ignore]
fn a_star_aaa() {
    let re = re!("(?<a>a*)(?<a3>aaa)");

    let m = re.mtch("aaa").unwrap();
    assert_eq!(m.capture(0).unwrap().range(), (0..3).into());
    assert_eq!(m.capture("a").unwrap().range(), (0..0).into()); // FIXME
    assert_eq!(m.capture("a3").unwrap().range(), (0..3).into()); // FIXME

    let m = re.mtch("aaaaa").unwrap();
    assert_eq!(m.capture(0).unwrap().range(), (0..5).into());
    assert_eq!(m.capture("a").unwrap().range(), (0..2).into()); // FIXME
    assert_eq!(m.capture("a3").unwrap().range(), (2..5).into()); // FIXME

    assert!(re.mtch("aa").is_none());
}

#[test]
fn a_star_star() {
    let re = re!("(?<sup>(?<sub>a*)*)");

    let m = re.mtch("aaa").unwrap();
    assert_eq!(m.capture(0).unwrap().range(), (0..3).into());
    assert_eq!(m.capture("sup").unwrap().range(), (0..3).into());
    assert_eq!(m.capture("sub").unwrap().range(), (3..3).into());
}
