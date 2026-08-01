use pretty_assertions::assert_eq;
use recz_adt::lit;
use recz_algo as real;
use recz_graph::{Graph, Tag};

#[test]
fn determine_0() {
    let nfa = Graph::new();
    nfa.node().connect(nfa.node());
    let dfa = real::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  no_0 {}
            ///}
        )
    );

    let nfa = Graph::new();
    let dfa = real::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  no_0 {}
            ///}
        )
    );
}

#[test]
fn determine_1() {
    let nfa = Graph::new();
    let a = nfa.node();
    let b = nfa.node();
    a.connect(b).add_tag(Tag::OpenGroup(1));
    let c = nfa.node();
    let d = nfa.node();
    b.connect(c).add_tag(Tag::OpenGroup(2));
    b.connect(d).add_tag(Tag::OpenGroup(3));
    let e = nfa.node();
    c.connect(e).add_tag(Tag::CloseGroup(2));
    d.connect(e).add_tag(Tag::CloseGroup(3));
    let f = nfa.node();
    e.connect(f).add_tag(Tag::CloseGroup(1));
    f.finalize();

    let dfa = real::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  no_0 { E / +g1,+g3,-g3,-g1,+g2,-g2 -> fi_1 }
            ///  fi_1 {}
            ///}
        )
    );
}

#[test]
fn determine_2() {
    let nfa = Graph::new();
    let a = nfa.node();
    let b = nfa.node();
    a.connect(b).add_tag(Tag::OpenGroup(1));
    let c = nfa.node();
    let d = nfa.node();
    b.connect(c).add_symbol(b'a');
    b.connect(d).add_symbol(b'b');
    let e = nfa.node();
    c.connect(e).add_tag(Tag::CloseGroup(2));
    d.connect(e).add_tag(Tag::CloseGroup(3));
    let f = nfa.node();
    e.connect(f).add_tag(Tag::CloseGroup(1));
    f.finalize();

    let dfa = real::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  no_0 {
            ///    'a' / +g1 -> no_1
            ///    'b' / +g1 -> no_2
            ///  }
            ///  no_1 { E / -g2,-g1 -> fi_3 }
            ///  no_2 { E / -g3,-g1 -> fi_3 }
            ///  fi_3 {}
            ///}
        )
    );
}
