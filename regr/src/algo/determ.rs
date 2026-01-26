use crate::graph::Graph;
use crate::node::Node;
use crate::tag::Inst;
use redt::{Map, Set};
use std::ops::Deref;
use std::rc::Rc;

pub fn determinate(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determinator = Determinator::new(nfa, &dfa);
    determinator.determinate();
    dfa
}

type Closure<'a> = Set<Node<'a>>;
type InstMap<'a> = Map<Node<'a>, Set<Inst>>;
type InstSet = Set<Inst>;

struct Determinator<'n, 'd> {
    nfa: &'n Graph,
    dfa: &'d Graph,
    conv_table: Map<Rc<Closure<'n>>, Node<'d>>,
    e_close_table: Map<Node<'n>, (Rc<Closure<'n>>, Rc<InstMap<'n>>)>,
    close_table: Map<(Node<'d>, u8), (Rc<Closure<'n>>, Rc<InstMap<'n>>)>,
}

impl<'n, 'd> Determinator<'n, 'd> {
    fn new(nfa: &'n Graph, dfa: &'d Graph) -> Self {
        Self {
            nfa,
            dfa,
            conv_table: Map::with_capacity(nfa.len() * 2),
            e_close_table: Map::with_capacity(nfa.len()),
            close_table: Map::with_capacity(nfa.len()),
        }
    }

    /// Creates a new DFA graph based on inner graph.
    fn determinate(&mut self) {
        assert!(self.dfa.is_empty(), "DFA must be empty");

        if self.nfa.is_empty() {
            return;
        }

        let (nfa_closure, inst_map) = self.e_close(self.nfa.start_node());
        let dfa_node = self.dfa.node();
        self.conv_table.insert(Rc::clone(&nfa_closure), dfa_node);

        let mut stack = Vec::with_capacity(self.nfa.len());
        stack.push((nfa_closure, dfa_node, inst_map));
        while let Some((nfa_closure, dfa_node, inst_map)) = stack.pop() {
            for sym in 0.. {
                let (sym_closure, new_inst_map) =
                    self.close(Rc::clone(&nfa_closure), dfa_node, sym);
                let new_dfa_node = self
                    .conv_table
                    .entry(Rc::clone(&sym_closure))
                    .or_insert_with(|| {
                        let new_dfa_node = self.dfa.node();
                        stack.push((Rc::clone(&sym_closure), new_dfa_node, new_inst_map));
                        new_dfa_node
                    });
                for inst in inst_map
            }
        }
    }

    fn close(
        &mut self,
        nfa_closure: Rc<Closure<'n>>,
        dfa_node: Node<'d>,
        symbol: u8,
    ) -> (Rc<Closure<'n>>, Rc<InstMap<'n>>) {
        if let Some((closure, inst_map)) = self.close_table.get(&(dfa_node, symbol)) {
            return (Rc::clone(closure), Rc::clone(inst_map));
        }

        let mut symbol_closure = Closure::default();
        let mut inst_map = InstMap::default();
        for node in nfa_closure.iter() {
            for (target, transitions) in node.targets() {
                let instructs = inst_map.entry(target).or_insert_with(Set::<Inst>::default);
                for tr in transitions {
                    if tr.contains(symbol) {
                        instructs.insert(tr.instruct());
                        symbol_closure.insert(target);
                    }
                }
            }
        }
        let symbol_closure = Rc::new(symbol_closure);
        let inst_map = Rc::new(inst_map);
        self.close_table.insert(
            (dfa_node, symbol),
            (Rc::clone(&symbol_closure), Rc::clone(&inst_map)),
        );
        (symbol_closure, inst_map)
    }

    fn e_close(&mut self, node: Node<'n>) -> (Rc<Closure<'n>>, Rc<InstMap<'n>>) {
        if let Some((e_closure, inst_map)) = self.e_close_table.get(&node) {
            return (Rc::clone(e_closure), Rc::clone(inst_map));
        }
        let mut inst_map = InstMap::<'n>::default();
        let mut e_closure = Closure::<'n>::with_capacity(self.nfa.len());
        let mut unvisited = Vec::with_capacity(self.nfa.len());
        unvisited.push(node);
        while let Some(node) = unvisited.pop() {
            e_closure.insert(node);
            for (target, transitions) in node.targets() {
                for tr in transitions {
                    if tr.is_epsilon() {
                        if !e_closure.contains(&target) {
                            unvisited.push(target);
                        }
                        let instructs = inst_map.entry(target).or_insert_with(Set::<Inst>::default);
                        instructs.insert(tr.instruct());
                    }
                }
            }
        }
        let e_closure = Rc::new(e_closure);
        let inst_map = Rc::new(inst_map);

        self.e_close_table
            .insert(node, (Rc::clone(&e_closure), Rc::clone(&inst_map)));
        (e_closure, inst_map)
    }
}
