//! Just a set of regular expressions from _wild_ internet.

use pretty_assertions::assert_eq;
use recz::re;

#[test]
fn zabb() {
    let re = re!(".abb|b");

    let m = re.mtch("zabb").unwrap();
    assert_eq!(m.as_str(), "zabb");
    assert_eq!(m.range(), (0..4).into());
    assert_eq!(m.capture(0).unwrap().as_str(), "zabb");
}
