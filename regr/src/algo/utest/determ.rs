use super::*;
use crate::graph::Graph;
use crate::tag::Inst::Nop;
use pretty_assertions::assert_eq;
use redt::{map, set};

#[test]
fn e_closure() {
    let nfa = Graph::new();

    let a = nfa.node();
    let b = nfa.node();
    let c = nfa.node();
    let d = nfa.node();
    let e = nfa.node();
    let f = nfa.node();

    a.connect(b, Nop);
    a.connect(c, Nop);
    b.connect(d, Nop).merge(1);
    c.connect(e, Nop);
    d.connect(f, Nop);
    e.connect(f, Nop);

    let dfa = Graph::new();

    let mut det = Determinator::new(&nfa, &dfa);
    assert_eq!(det.e_close(a).nodes, set![a, c, e, f, b]);
    assert_eq!(det.e_close(b).nodes, set![b]);
    assert_eq!(det.e_close(c).nodes, set![c, e, f]);
    assert_eq!(det.e_close(d).nodes, set![d, f]);
    assert_eq!(det.e_close(e).nodes, set![e, f]);
    assert_eq!(det.e_close(f).nodes, set![f]);

    f.connect(b, Nop);
    let mut det = Determinator::new(&nfa, &dfa);
    assert_eq!(det.e_close(f).nodes, set![f, b]);

    f.connect(c, Nop);
    let mut det = Determinator::new(&nfa, &dfa);
    assert_eq!(det.e_close(f).nodes, set![f, c, e, b]);
}

#[test]
#[ignore]
fn e_closure_with_tags() {
    let nfa = Graph::new();

    let q = nfa.node();
    let a = nfa.node();
    let b = nfa.node();
    let c = nfa.node();
    let d = nfa.node();
    let e = nfa.node();
    let f = nfa.node();
    let g = nfa.node();

    let t0 = nfa.tag();
    let t1 = nfa.tag();
    let t2 = nfa.tag();
    let t3 = nfa.tag();

    q.connect(a, t0.pos_inst());
    a.connect(b, t1.pos_inst());
    a.connect(c, t2.pos_inst());
    b.connect(d, Nop).merge(1);
    c.connect(e, Nop).merge(2);
    d.connect(f, t2.neg_inst());
    e.connect(f, t1.neg_inst());
    f.connect(g, t3.pos_inst());

    let dfa = Graph::new();

    let mut det = Determinator::new(&nfa, &dfa);
    let e_closure = det.e_close(q);
    assert_eq!(e_closure.nodes, set![q, a, c, b]);
    assert_eq!(
        e_closure.inst_map,
        map! {
            a => set![t0.pos_inst()],
            b => set![t1.pos_inst(), t0.pos_inst()],
            c => set![t2.pos_inst(), t0.pos_inst()],
        }
    );

    // assert_eq!(det.e_closure(d), set![d, f, g]);
    // assert_eq!(
    //     Map::from_iter(det.inst_map.iter().map(|(k, v)| (*k, v.clone()))),
    //     map! {
    //         a => set![Inst::WritePos(0, 0)],
    //         b => set![Inst::WritePos(1, 1), Inst::WritePos(0, 0)],
    //         c => set![Inst::WritePos(2, 2), Inst::WritePos(0, 0)],
    //         f => set![Inst::InvalidateTag(2)],
    //         g => set![Inst::WritePos(3, 3), Inst::InvalidateTag(2)],
    //     }
    // );

    // assert_eq!(det.e_closure(e), set![e, f, g]);
    // assert_eq!(
    //     Map::from_iter(det.inst_map.iter().map(|(k, v)| (*k, v.clone()))),
    //     map! {
    //         a => set![Inst::WritePos(0, 0)],
    //         b => set![Inst::WritePos(1, 1), Inst::WritePos(0, 0)],
    //         c => set![Inst::WritePos(2, 2), Inst::WritePos(0, 0)],
    //         f => set![Inst::InvalidateTag(2), Inst::InvalidateTag(1)],
    //         g => set![Inst::WritePos(3, 3), Inst::InvalidateTag(2), Inst::InvalidateTag(1)],
    //     }
    // );
}
