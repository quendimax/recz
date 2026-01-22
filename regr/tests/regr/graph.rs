use pretty_assertions::assert_eq;
use redt::RangeU8;
use redt::lit;
use regr::{Graph, Inst::Nop, Tag, TagBank};

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

    a.connect(b, Nop).merge(RangeU8::new(b'a', u8::MAX));
    a.connect(b, Nop);
    b.connect(c, Nop);
    c.connect(d, Nop).merge(b'c');
    b.connect(a, Nop);
    d.connect(a, Nop);
    d.connect(b, Nop);
    d.connect(c, Nop);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///node(0) {
            ///    ['a'-FFh] -> node(1)
            ///}
            ///node(1) {
            ///    [Epsilon] -> node(2)
            ///    [Epsilon] -> node(0)
            ///}
            ///node(2) {
            ///    ['c'] -> node(3)
            ///}
            ///node(3) {
            ///    [Epsilon] -> node(0)
            ///    [Epsilon] -> node(1)
            ///    [Epsilon] -> node(2)
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
    n0.connect(n1, Nop).merge(RangeU8::from(b'a'..=b'b'));
    n0.connect(n1, Nop).merge(RangeU8::from(b'd'..=b'z'));
    n1.connect(n2, Nop);
    n1.connect(n4, Nop);
    n2.connect(n3, Nop).merge(b'a');
    n3.connect(n4, Nop);
    n3.connect(n2, Nop);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///node(0) {
            ///    ['a'-'b' | 'd'-'z'] -> node(1)
            ///}
            ///node(1) {
            ///    [Epsilon] -> node(2)
            ///    [Epsilon] -> node(4)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    [Epsilon] -> node(4)
            ///    [Epsilon] -> node(2)
            ///}
            ///node(4) {}
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
    n0.connect(n2, Nop);
    n0.connect(n5, Nop);
    n2.connect(n3, Nop).merge(b'a');
    n3.connect(n4, Nop).merge(b'b');
    n4.connect(n1, Nop);
    n5.connect(n6, Nop).merge(b'c');
    n6.connect(n7, Nop).merge(b'd');
    n7.connect(n1, Nop);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(2)
            ///    [Epsilon] -> node(5)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    ['b'] -> node(4)
            ///}
            ///node(4) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(1) {}
            ///node(5) {
            ///    ['c'] -> node(6)
            ///}
            ///node(6) {
            ///    ['d'] -> node(7)
            ///}
            ///node(7) {
            ///    [Epsilon] -> node(1)
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
    a.connect(b, Nop).merge(1);
    b.connect(b, Nop).merge(3);
    b.connect(c, Nop).merge(1);
    assert_eq!(
        format!("{graph:?}"),
        lit!(
            ///node(0) {
            ///    [1] -> node(1)
            ///}
            ///node(1) {
            ///    [3] -> self
            ///    [1] -> node(2)
            ///}
            ///node(2) {}
        )
    );
}

#[test]
fn graph_tags() {
    let graph = Graph::new();
    let mut tag_bank = TagBank::default();
    graph.add_tag_group(0, tag_bank.absolute(), tag_bank.absolute());
    graph.add_tag_group(1, tag_bank.absolute(), tag_bank.absolute());

    let mut tag_bank = TagBank::default();
    assert_eq!(
        graph.tag_group(0),
        Some((tag_bank.absolute(), tag_bank.absolute()))
    );
    assert_eq!(graph.tag_group(2), None);
    let mut tag_groups = graph.tag_groups().collect::<Vec<_>>();
    tag_groups.sort_by_key(|(id, _)| *id);
    assert_eq!(
        tag_groups,
        [
            (
                0,
                (
                    Tag::Absolute { id: 0, reg: 0 },
                    Tag::Absolute { id: 1, reg: 1 },
                ),
            ),
            (
                1,
                (
                    Tag::Absolute { id: 2, reg: 2 },
                    Tag::Absolute { id: 3, reg: 3 },
                ),
            ),
        ]
    );
}
