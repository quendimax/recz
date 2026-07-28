use recz_algo::VisitResult::*;
use recz_graph::Graph;

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
    recz_algo::visit_nodes(a, |node| {
        vec.push(node);
        Recurse
    });
    vec.sort();
    assert_eq!(vec, [a, b, c, d]);

    let mut vec = Vec::new();
    recz_algo::visit_nodes(e, |node| {
        vec.push(node);
        Recurse
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
    recz_algo::visit_nodes(a, |node| {
        vec.push(node);
        if node == b { Continue } else { Recurse }
    });
    vec.sort();
    assert_eq!(vec, [a, b, c]);

    let mut vec = Vec::new();
    recz_algo::visit_nodes(a, |node| {
        vec.push(node);
        if node == e { Continue } else { Recurse }
    });
    vec.sort();
    assert_eq!(vec, [a, b, c, d, e]);
}

#[test]
fn for_each_edge() {
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
    recz_algo::visit_edges(a, |source, _, target| {
        vec.push((source, target));
        Recurse
    });
    vec.sort();
    assert_eq!(vec, [(a, b), (a, d), (b, c), (c, a)]);

    let mut vec = Vec::new();
    recz_algo::visit_edges(e, |source, _, target| {
        vec.push((source, target));
        Recurse
    });
    vec.sort();
    assert_eq!(vec, [(a, b), (a, d), (b, c), (c, a), (e, a)]);
}

#[test]
fn for_each_edge_in_tree() {
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
    recz_algo::visit_edges(a, |source, _, target| {
        vec.push((source, target));
        if target == b { Continue } else { Recurse }
    });
    vec.sort();
    assert_eq!(vec, [(a, b), (a, c)]);

    let mut vec = Vec::new();
    recz_algo::visit_edges(a, |source, _, target| {
        vec.push((source, target));
        if target == e { Continue } else { Recurse }
    });
    vec.sort();
    assert_eq!(vec, [(a, b), (a, c), (b, d), (b, e)]);
}
