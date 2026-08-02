use pretty_assertions::assert_eq;
use recz_adt::lit;
use recz_algo as real;
use recz_graph::{Graph, Tag, Translator};
use recz_syntax::{Parser, codec::Utf8Codec};

fn parse(s: &str) -> String {
    let parser = Parser::new(Utf8Codec);
    let hir = parser.parse(s).unwrap();
    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());
    let dfa = real::determine(nfa);
    format!("{dfa}")
}

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
            ///  no_0 { EPS / +g1,+g3,-g3,-g1,+g2,-g2 -> fi_1 }
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
            ///  no_1 { EPS / -g2,-g1 -> fi_3 }
            ///  no_2 { EPS / -g3,-g1 -> fi_3 }
            ///  fi_3 {}
            ///}
        )
    );
}

#[test]
fn determine_3() {
    assert_eq!(
        parse(r"hello"),
        lit!(
            ///graph {
            ///  no_0 { 'h' -> no_1 }
            ///  no_1 { 'e' -> no_2 }
            ///  no_2 { 'l' -> no_3 }
            ///  no_3 { 'l' -> no_4 }
            ///  no_4 { 'o' -> no_5 }
            ///  no_5 { EPS -> fi_6 }
            ///  fi_6 {}
            ///}
        )
    );
}

#[test]
fn determine_4() {
    assert_eq!(
        parse(r"aa*"),
        lit!(
            ///graph {
            ///  no_0 { 'a' -> no_1 }
            ///  no_1 {
            ///    'a' -> no_2
            ///    EPS -> fi_3
            ///  }
            ///  no_2 {
            ///    'a' -> self
            ///    EPS -> fi_3
            ///  }
            ///  fi_3 {}
            ///}
        )
    );
}
