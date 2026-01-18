use super::*;
use crate::{Arena, Graph, Inst};
use pretty_assertions::assert_eq;
use redt::{map, set};

#[test]
fn e_closure() {
    let mut nfa_arena = Arena::new();
    let nfa = Graph::new_in(&mut nfa_arena);

    let a = nfa.node();
    let b = nfa.node();
    let c = nfa.node();
    let d = nfa.node();
    let e = nfa.node();
    let f = nfa.node();

    a.connect(b);
    a.connect(c);
    b.connect(d).merge(1);
    c.connect(e);
    d.connect(f);
    e.connect(f);

    let mut dfa_arena = Arena::new();
    let dfa = Graph::new_in(&mut dfa_arena);

    let mut det = Determinizer::new(&nfa, &dfa);
    assert_eq!(det.e_closure(a), set![a, b, c, e, f]);
    assert_eq!(det.e_closure(b), set![b]);
    assert_eq!(det.e_closure(c), set![c, e, f]);
    assert_eq!(det.e_closure(d), set![d, f]);
    assert_eq!(det.e_closure(e), set![e, f]);
    assert_eq!(det.e_closure(f), set![f]);

    f.connect(b);
    assert_eq!(det.e_closure(f), set![f, b]);

    f.connect(c);
    assert_eq!(det.e_closure(f), set![f, b, c, e]);

    assert!(det.inst_map.is_empty());
}

#[test]
fn e_closure_with_tags() {
    let mut nfa_arena = Arena::new();
    let nfa = Graph::new_in(&mut nfa_arena);

    let q = nfa.node();
    let a = nfa.node();
    let b = nfa.node();
    let c = nfa.node();
    let d = nfa.node();
    let e = nfa.node();
    let f = nfa.node();
    let g = nfa.node();

    q.connect(a).merge_instruct(Inst::WritePos(0, 0), None);
    a.connect(b).merge_instruct(Inst::WritePos(1, 1), None);
    a.connect(c).merge_instruct(Inst::WritePos(2, 2), None);
    b.connect(d).merge(1);
    c.connect(e).merge(2);
    d.connect(f).merge_instruct(Inst::InvalidateTag(2), None);
    e.connect(f).merge_instruct(Inst::InvalidateTag(1), None);
    f.connect(g).merge_instruct(Inst::WritePos(3, 3), None);

    let mut dfa_arena = Arena::new();
    let dfa = Graph::new_in(&mut dfa_arena);

    let mut det = Determinizer::new(&nfa, &dfa);
    assert_eq!(det.e_closure(q), set![q, a, b, c]);
    assert_eq!(
        det.inst_map,
        map! {
            a => set![Inst::WritePos(0, 0)],
            b => set![Inst::WritePos(1, 1), Inst::WritePos(0, 0)],
            c => set![Inst::WritePos(2, 2), Inst::WritePos(0, 0)],
        }
    );

    assert_eq!(det.e_closure(d), set![d, f, g]);
    assert_eq!(
        Map::from_iter(det.inst_map.iter().map(|(k, v)| (*k, v.clone()))),
        map! {
            a => set![Inst::WritePos(0, 0)],
            b => set![Inst::WritePos(1, 1), Inst::WritePos(0, 0)],
            c => set![Inst::WritePos(2, 2), Inst::WritePos(0, 0)],
            f => set![Inst::InvalidateTag(2)],
            g => set![Inst::WritePos(3, 3), Inst::InvalidateTag(2)],
        }
    );

    assert_eq!(det.e_closure(e), set![e, f, g]);
    assert_eq!(
        Map::from_iter(det.inst_map.iter().map(|(k, v)| (*k, v.clone()))),
        map! {
            a => set![Inst::WritePos(0, 0)],
            b => set![Inst::WritePos(1, 1), Inst::WritePos(0, 0)],
            c => set![Inst::WritePos(2, 2), Inst::WritePos(0, 0)],
            f => set![Inst::InvalidateTag(2), Inst::InvalidateTag(1)],
            g => set![Inst::WritePos(3, 3), Inst::InvalidateTag(2), Inst::InvalidateTag(1)],
        }
    );
}
