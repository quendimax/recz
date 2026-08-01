use pretty_assertions::assert_eq;
use recz_adt::lit;
use recz_algo as real;
use recz_graph::Graph;

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
}

#[test]
fn determine_1() {
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
