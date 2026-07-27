use recz_adt::range;
use recz_graph::Graph;

#[test]
fn verify_dfa() {
    let nfa = Graph::new();
    let a = nfa.node();
    let b = nfa.node();
    let c = nfa.node();
    let d = nfa.node();
    a.connect(a).add_symbols(range(1, 255));
    a.connect(b).add_symbol(0);
    b.connect(c).add_symbol(b'a');
    c.connect(d).add_symbol(b'b');
    assert!(recz_algo::verify_dfa(&nfa));

    a.connect(b).add_symbol(b'a');
    assert!(!recz_algo::verify_dfa(&nfa));
}

#[test]
fn verify_dfa_with_epsilon() {
    let nfa = Graph::new();
    nfa.node().connect(nfa.node());
    assert!(!recz_algo::verify_dfa(&nfa));
}
