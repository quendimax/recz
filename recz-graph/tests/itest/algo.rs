use pretty_assertions::assert_eq;
use recz_adt::lit;
use recz_graph::{Graph, algo};
use recz_syntax::{Parser, Translator, codec::Utf8Codec};

#[test]
fn determine_0() {
    let nfa = Graph::new();
    nfa.node().connect(nfa.node());
    let dfa = algo::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  no_0 {}
            ///}
        )
    );

    let nfa = Graph::new();
    let dfa = algo::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {}
        )
    );
}

#[test]
fn determine_1() {
    let nfa = Graph::new();
    let _gr_0 = nfa.group(0u32);
    let gr_1 = nfa.group(1u32);
    let gr_2 = nfa.group("hello");
    let gr_3 = nfa.group("hlo");
    let a = nfa.node();
    let b = nfa.node();
    a.connect(b).add_tag(gr_1.open_tag());
    let c = nfa.node();
    let d = nfa.node();
    b.connect(c).add_tag(gr_2.open_tag());
    b.connect(d).add_tag(gr_3.open_tag());
    let e = nfa.node();
    c.connect(e).add_tag(gr_2.close_tag());
    d.connect(e).add_tag(gr_3.close_tag());
    let f = nfa.node();
    e.connect(f).add_tag(gr_1.close_tag());
    f.finalize();

    let dfa = algo::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  fi_0 { EPS / +g1,+g2,+g3,-g1,-g2,-g3 -> eg_1 }
            ///  eg_1 {}
            ///}
        )
    );
}

#[test]
fn determine_2() {
    let nfa = Graph::new();
    let _gr_0 = nfa.group(0u32);
    let gr_1 = nfa.group(1u32);
    let gr_2 = nfa.group("hello");
    let gr_3 = nfa.group("hlo");
    let a = nfa.node();
    let b = nfa.node();
    a.connect(b).add_tag(gr_1.open_tag());
    let c = nfa.node();
    let d = nfa.node();
    b.connect(c).add_symbol(b'a');
    b.connect(d).add_symbol(b'b');
    let e = nfa.node();
    c.connect(e).add_tag(gr_2.close_tag());
    d.connect(e).add_tag(gr_3.close_tag());
    let f = nfa.node();
    e.connect(f).add_tag(gr_1.close_tag());
    f.finalize();

    let dfa = algo::determine(nfa);
    assert_eq!(
        format!("{dfa}"),
        lit!(
            ///graph {
            ///  no_0 {
            ///    'a' / +g1 -> fi_1
            ///    'b' / +g1 -> fi_2
            ///  }
            ///  fi_1 { EPS / -g1,-g2 -> eg_3 }
            ///  fi_2 { EPS / -g1,-g3 -> eg_3 }
            ///  eg_3 {}
            ///}
        )
    );
}

fn parse(s: &str) -> String {
    let parser = Parser::new(Utf8Codec);
    let hir = parser.parse(s).unwrap();
    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());
    let dfa = algo::determine(nfa);
    format!("{dfa}")
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
            ///  no_4 { 'o' -> fi_5 }
            ///  fi_5 { EPS -> eg_6 }
            ///  eg_6 {}
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
            ///  no_0 { 'a' -> fi_1 }
            ///  fi_1 {
            ///    'a' -> fi_2
            ///    EPS -> eg_3
            ///  }
            ///  fi_2 {
            ///    'a' -> self
            ///    EPS -> eg_3
            ///  }
            ///  eg_3 {}
            ///}
        )
    );
}

#[test]
fn ambiguity_0() {
    assert_eq!(
        parse(r"a*(?D<0>a*)"),
        lit!(
            ///graph {
            ///  fi_0 {
            ///    'a' / +g0 -> fi_1
            ///    EPS / +g0,-g0 -> eg_2
            ///  }
            ///  fi_1 {
            ///    'a' / +g0 -> self
            ///    EPS / +g0,-g0 -> eg_2
            ///  }
            ///  eg_2 {}
            ///}
        )
    );
}

#[test]
fn ambiguity_1() {
    assert_eq!(
        parse(r"(?D<0>a*)a*"),
        lit!(
            ///graph {
            ///  fi_0 {
            ///    'a' / +g0,-g0 -> fi_1
            ///    EPS / +g0,-g0 -> eg_2
            ///  }
            ///  fi_1 {
            ///    'a' / -g0 -> self
            ///    EPS / -g0 -> eg_2
            ///  }
            ///  eg_2 {}
            ///}
        )
    );
}

#[test]
fn ambiguity_2() {
    assert_eq!(
        parse(r"(?D<1>a)a*"),
        lit!(
            ///graph {
            ///  no_0 { 'a' / +g0 -> fi_1 }
            ///  fi_1 {
            ///    'a' / -g0 -> fi_2
            ///    EPS / -g0 -> eg_3
            ///  }
            ///  fi_2 {
            ///    'a' -> self
            ///    EPS -> eg_3
            ///  }
            ///  eg_3 {}
            ///}
        )
    );
}

#[test]
fn ambiguity_3() {
    assert_eq!(
        parse(r"a*(?D<1>a)"),
        lit!(
            ///graph {
            ///  no_0 { 'a' / +g0 -> fi_1 }
            ///  fi_1 {
            ///    'a' / +g0 -> self
            ///    EPS / -g0 -> eg_2
            ///  }
            ///  eg_2 {}
            ///}
        )
    );
}
