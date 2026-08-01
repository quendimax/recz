use pretty_assertions::{assert_eq, assert_ne};
use recz_adt::range;
use recz_graph::Graph;

#[test]
fn node_copy_and_clone() {
    let graph = Graph::new();
    let node = graph.node();
    #[allow(clippy::clone_on_copy)]
    let cloned_node = node.clone();
    let copied_node = node;
    assert_eq!(node.nid(), cloned_node.nid());
    assert_eq!(node.gid(), cloned_node.gid());
    assert_eq!(node.uid(), cloned_node.uid());
    assert_eq!(node.nid(), copied_node.nid());
    assert_eq!(node.gid(), copied_node.gid());
    assert_eq!(node.uid(), copied_node.uid());
}

#[test]
fn node_id() {
    let graph_0 = Graph::new();
    let a = graph_0.node();
    let b = graph_0.node();

    assert_eq!(a.nid(), 0);
    assert_eq!(a.uid(), (a.gid() as u64) << (u64::BITS / 2));

    assert_eq!(b.nid(), 1);
    assert_eq!(b.uid(), ((b.gid() as u64) << (u64::BITS / 2)) | 1);

    let graph_1 = Graph::new();
    let c = graph_1.node();
    let d = graph_1.node();

    assert_eq!(c.nid(), 0);
    assert_ne!(c.gid(), a.gid());
    assert_eq!(c.uid(), (c.gid() as u64) << (u64::BITS / 2));

    assert_eq!(d.nid(), 1);
    assert_ne!(d.gid(), a.gid());
    assert_eq!(d.uid(), ((c.gid() as u64) << (u64::BITS / 2)) | 1);
}

#[test]
fn node_partial_eq() {
    let graph = Graph::new();
    let node_1 = graph.node();
    assert_ne!(node_1, graph.node());

    let graph = Graph::new();
    let node_2 = graph.node();
    assert_ne!(node_1, node_2);
}

#[test]
fn node_connect_nfa() {
    let graph = Graph::new();
    let node_a = graph.node();
    let node_b = graph.node();
    let node_c = graph.node();
    node_a.connect(node_b).add_symbol(b'a');
    node_a.connect(node_c).add_symbol(b'a');
    node_a.connect(node_c).add_symbol(b'a');
    node_c.connect(node_a);
}

#[test]
fn node_connect_dfa() {
    let graph = Graph::new();
    let node_a = graph.node();
    let node_b = graph.node();
    node_a.connect(node_b).add_symbol(b'a');
}

#[test]
#[should_panic(expected = "only nodes belonging to the same graph can be joined")]
fn node_connect_panics() {
    let graph_a = Graph::new();
    let graph_b = Graph::new();
    let node_a = graph_a.node();
    let node_b = graph_b.node();
    node_a.connect(node_b);
}

#[test]
fn node_symbol_targets() {
    let graph = Graph::new();
    let a = graph.node();
    let b = graph.node();
    let c = graph.node();
    let d = graph.node();

    a.connect(b).add_symbols(range(b'a', u8::MAX));
    a.connect(b);
    b.connect(c);
    c.connect(d).add_symbol(b'c');
    b.connect(a);
    d.connect(a);
    d.connect(b);
    d.connect(c);

    assert_eq!(
        a.targets().map(|(_, node)| node).collect::<Vec<_>>(),
        vec![b]
    );
    assert_eq!(
        c.targets().map(|(_, node)| node).collect::<Vec<_>>(),
        vec![d]
    );
}

#[test]
fn node_modify_tr_during_iteration() {
    let graph = Graph::new();
    let a = graph.node();
    let b = graph.node();
    a.connect(b).add_symbol(b'c');

    for _ in a.targets() {
        a.connect(b).add_symbol(b'a');
    }
}

#[test]
fn node_finalize() {
    let graph = Graph::new();
    let a = graph.node();
    assert_eq!(format!("{a:?}"), "no_0");
    a.finalize();
    assert_eq!(format!("{a:?}"), "fi_0");
    a.definalize();
    assert_eq!(format!("{a:?}"), "no_0");
}

#[test]
fn node_fmt_debug() {
    let graph = Graph::new();
    let a = graph.node();
    let b = graph.node();
    let c = graph.node().finalize();
    assert_eq!(format!("{a:?}"), "no_0");
    assert_eq!(format!("{b:?}"), "no_1");
    assert_eq!(format!("{c:?}"), "fi_2");
}
