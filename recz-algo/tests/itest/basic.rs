use pretty_assertions::assert_eq;
use recz_algo::VisitWay::*;
use recz_graph::{Graph, Tag};

#[test]
fn visit_nodes() {
    let gr = Graph::new();
    let a = gr.node();
    let b = gr.node();
    let c = gr.node();
    let d = gr.node();
    let e = gr.node();

    a.connect(b);
    b.connect(c);
    c.connect(a);
    a.connect(d);
    e.connect(a);

    let mut vec = Vec::new();
    recz_algo::visit_nodes(a, |node, _| {
        vec.push(node);
        Descend
    });
    vec.sort();
    assert_eq!(vec, [a, b, c, d]);

    let mut vec = Vec::new();
    recz_algo::visit_nodes(e, |node, _| {
        vec.push(node);
        Descend
    });
    vec.sort();
    assert_eq!(vec, [a, b, c, d, e]);
}

#[test]
fn visit_nodes_in_tree() {
    let gr = Graph::new();
    let a = gr.node();
    let b = gr.node();
    let c = gr.node();
    let d = gr.node();
    let e = gr.node();
    let f = gr.node();
    let g = gr.node();

    a.connect(b);
    a.connect(c);

    b.connect(d);
    b.connect(e);

    e.connect(f);
    e.connect(g);

    let mut vec = Vec::new();
    recz_algo::visit_nodes(a, |node, _| {
        vec.push(node);
        if node == b { Sideways } else { Descend }
    });
    vec.sort();
    assert_eq!(vec, [a, b, c]);

    let mut vec = Vec::new();
    recz_algo::visit_nodes(a, |node, _| {
        vec.push(node);
        if node == e { Sideways } else { Descend }
    });
    vec.sort();
    assert_eq!(vec, [a, b, c, d, e]);
}

#[test]
fn e_close() {
    let gr = Graph::new();
    let a = gr.node();
    let b = gr.node();
    let c = gr.node();
    let d = gr.node();
    let e = gr.node();
    let f = gr.node();

    a.connect(b).add_tag(Tag::OpenGroup(1));
    a.connect(c).add_tag(Tag::OpenGroup(2));
    b.connect(d).add_tag(Tag::CloseGroup(1));
    c.connect(e).add_tag(Tag::CloseGroup(2));
    e.connect(f);
    d.connect(f);

    let closure = recz_algo::e_close([a]);
    assert_eq!(
        format!("{closure:?}"),
        concat!(
            "({node(0), node(1), node(2), node(3), node(4), node(5)}, ",
            "{node(0): {}, node(2): {+g2}, node(4): {-g2, +g2}, ",
            "node(5): {-g2, +g2, -g1, +g1}, node(1): {+g1}, node(3): {-g1, +g1}})"
        )
    );
}
