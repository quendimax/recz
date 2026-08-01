use pretty_assertions::assert_eq;
use recz_adt::RangeU8;
use recz_adt::lit;
use recz_graph::Graph;

#[test]
fn graph_node() {
    let graph = Graph::new();
    assert_eq!(graph.node().nid(), 0);
    assert_eq!(graph.node().nid(), 1);
    assert_eq!(graph.node().nid(), 2);
    drop(graph);

    let graph = Graph::new();
    assert_eq!(graph.node().nid(), 0);
    assert_eq!(graph.node().nid(), 1);
    assert_eq!(graph.node().nid(), 2);
}

#[test]
fn graph_start_node() {
    let graph = Graph::new();
    assert_eq!(graph.start_node().nid(), 0);
    assert_eq!(graph.node().nid(), 1);
    assert_eq!(graph.node().nid(), 2);
    drop(graph);

    let graph = Graph::new();
    assert_eq!(graph.node(), graph.start_node());
    assert_eq!(graph.start_node().nid(), 0);
    assert_eq!(graph.node().nid(), 1);
    assert_eq!(graph.node().nid(), 2);
    assert_eq!(graph.start_node().nid(), 0);
}

#[test]
fn graph_display_fmt_0() {
    let graph = Graph::new();
    let a = graph.node();
    let b = graph.node();
    let c = graph.node();
    let d = graph.node();

    a.connect(b).add_symbols(RangeU8::new(b'a', u8::MAX));
    a.connect(b);
    b.connect(c);
    c.connect(d).add_symbol(b'c');
    b.connect(a);
    d.connect(a);
    d.connect(b);
    d.connect(c);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///graph {
            ///  no_0 { 'a'-'\xFF' -> no_1 }
            ///  no_1 {
            ///    E -> no_2
            ///    E -> no_0
            ///  }
            ///  no_2 { 'c' -> no_3 }
            ///  no_3 {
            ///    E -> no_0
            ///    E -> no_1
            ///    E -> no_2
            ///  }
            ///}
        )
    );
}

#[test]
fn graph_display_fmt_1() {
    let graph = Graph::new();
    let n0 = graph.node();
    let n1 = graph.node();
    let n2 = graph.node();
    let n3 = graph.node();
    let n4 = graph.node();
    n0.connect(n1).add_symbols(RangeU8::from(b'a'..=b'b'));
    n0.connect(n1).add_symbols(RangeU8::from(b'd'..=b'z'));
    n1.connect(n2);
    n1.connect(n4);
    n2.connect(n3).add_symbols(b'a');
    n3.connect(n4);
    n3.connect(n2);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///graph {
            ///  no_0 { 'a'-'b' | 'd'-'z' -> no_1 }
            ///  no_1 {
            ///    E -> no_2
            ///    E -> no_4
            ///  }
            ///  no_2 { 'a' -> no_3 }
            ///  no_3 {
            ///    E -> no_4
            ///    E -> no_2
            ///  }
            ///  no_4 {}
            ///}
        )
    );
}

#[test]
fn graph_display_fmt_2() {
    let graph = Graph::new();
    let n0 = graph.node();
    let n1 = graph.node();
    let n2 = graph.node();
    let n3 = graph.node();
    let n4 = graph.node();
    let n5 = graph.node();
    let n6 = graph.node();
    let n7 = graph.node();
    n0.connect(n2);
    n0.connect(n5);
    n2.connect(n3).add_symbol(b'a');
    n3.connect(n4).add_symbol(b'b');
    n4.connect(n1);
    n5.connect(n6).add_symbol(b'c');
    n6.connect(n7).add_symbol(b'd');
    n7.connect(n1);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///graph {
            ///  no_0 {
            ///    E -> no_2
            ///    E -> no_5
            ///  }
            ///  no_1 {}
            ///  no_2 { 'a' -> no_3 }
            ///  no_3 { 'b' -> no_4 }
            ///  no_4 { E -> no_1 }
            ///  no_5 { 'c' -> no_6 }
            ///  no_6 { 'd' -> no_7 }
            ///  no_7 { E -> no_1 }
            ///}
        )
    );
}

#[test]
fn graph_display_fmt_3() {
    let graph = Graph::new();
    let a = graph.node();
    let b = graph.node();
    let c = graph.node();
    a.connect(b).add_symbol(1);
    b.connect(b).add_symbol(3);
    b.connect(c).add_symbol(1);
    assert_eq!(
        format!("{graph:?}"),
        lit!(
            ///graph {
            ///  no_0 { '\x01' -> no_1 }
            ///  no_1 {
            ///    '\x03' -> self
            ///    '\x01' -> no_2
            ///  }
            ///  no_2 {}
            ///}
        )
    );
}
