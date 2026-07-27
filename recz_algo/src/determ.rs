use recz_adt::{Map, Set};
use regr::{Graph, Node, Tag};
use std::rc::Rc;

pub fn determinate(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determinator = Determinator::new(nfa, &dfa);
    determinator.determinate();
    dfa
}

struct Closure<'a> {
    nodes: Set<Node<'a>>,
    inst_map: Map<Node<'a>, Set<Inst>>,
}

impl Closure<'_> {
    fn new() -> Self {
        Self {
            nodes: Set::new(),
            inst_map: Map::new(),
        }
    }
}

impl std::hash::Hash for Closure<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nodes.hash(state);
    }
}

impl PartialEq for Closure<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl Eq for Closure<'_> {}

type InstSet = Set<Inst>;

struct Determinator<'n, 'd> {
    nfa: &'n Graph,
    dfa: &'d Graph,
    conv_table: Map<Rc<Closure<'n>>, Node<'d>>,
    e_close_table: Map<Node<'n>, Rc<Closure<'n>>>,
    close_table: Map<(Node<'d>, u8), (Rc<Closure<'n>>, Rc<InstSet>)>,
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

        let nfa_closure = self.e_close(self.nfa.start_node());
        let dfa_node = self.dfa.node();
        self.conv_table.insert(Rc::clone(&nfa_closure), dfa_node);

        let mut stack = Vec::with_capacity(self.nfa.len());
        stack.push((nfa_closure, dfa_node));
        while let Some((nfa_closure, dfa_node)) = stack.pop() {
            for sym in 0.. {
                let (sym_closure, insts) = self.close(Rc::clone(&nfa_closure), dfa_node, sym);
                let new_dfa_node = self
                    .conv_table
                    .entry(Rc::clone(&sym_closure))
                    .or_insert_with(|| {
                        let new_dfa_node = self.dfa.node();
                        stack.push((Rc::clone(&sym_closure), new_dfa_node));
                        new_dfa_node
                    });
                for inst in insts.iter() {
                    dfa_node.connect(*new_dfa_node, *inst).merge(sym);
                }
            }
        }
    }

    fn close(
        &mut self,
        nfa_closure: Rc<Closure<'n>>,
        dfa_node: Node<'d>,
        symbol: u8,
    ) -> (Rc<Closure<'n>>, Rc<Set<Inst>>) {
        if let Some((s_closure, insts)) = self.close_table.get(&(dfa_node, symbol)) {
            return (Rc::clone(s_closure), Rc::clone(insts));
        }

        let mut insts = Set::<Inst>::default();
        let mut s_closure = Closure::new();
        for node in nfa_closure.nodes.iter() {
            for (target, transitions) in node.targets() {
                for tr in transitions {
                    if tr.contains(symbol) {
                        s_closure.nodes.insert(target);
                        if let Some(inst_set) = s_closure.inst_map.get(node) {
                            insts.extend(inst_set);
                        }

                        let e_s_closure = self.e_close(target);
                        s_closure.nodes.extend(e_s_closure.nodes.iter());
                        for (node, inst_set) in &e_s_closure.inst_map {
                            s_closure
                                .inst_map
                                .entry(*node)
                                .or_default()
                                .extend(inst_set);
                        }
                    }
                }
            }
        }
        let s_closure = Rc::new(s_closure);
        let insts = Rc::new(insts);
        self.close_table.insert(
            (dfa_node, symbol),
            (Rc::clone(&s_closure), Rc::clone(&insts)),
        );
        (s_closure, insts)
    }

    fn e_close(&mut self, node: Node<'n>) -> Rc<Closure<'n>> {
        if let Some(e_closure) = self.e_close_table.get(&node) {
            return Rc::clone(e_closure);
        }
        let mut e_closure = Closure::<'n>::new();
        let mut unvisited = Vec::with_capacity(self.nfa.len());
        unvisited.push(node);
        while let Some(node) = unvisited.pop() {
            e_closure.nodes.insert(node);
            for (target, transitions) in node.targets() {
                for tr in transitions {
                    if tr.is_epsilon() {
                        if !e_closure.nodes.contains(&target) {
                            unvisited.push(target);
                        }
                        let instructs = e_closure.inst_map.entry(target).or_default();
                        instructs.insert(tr.instruct());
                    }
                }
            }
        }

        let e_closure = Rc::new(e_closure);
        self.e_close_table.insert(node, Rc::clone(&e_closure));

        e_closure
    }
}

#[cfg(test)]
#[path = "utest/determ.rs"]
mod utest;
