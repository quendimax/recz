use crate::basic::e_close;
use recz_adt::{Map, OrdSet};
use recz_graph::{Graph, Node, Tag};

pub fn determinate(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determinator = Determinator::new(nfa, &dfa);
    determinator.determinate();
    dfa
}

struct Determinator<'d, 'n> {
    nfa: &'n Graph,
    dfa: &'d Graph,
}

impl<'d, 'n> Determinator<'d, 'n> {
    fn new(nfa: &'n Graph, dfa: &'d Graph) -> Self {
        assert_ne!(nfa.gid(), dfa.gid());
        assert!(dfa.is_empty(), "DFA must be empty");
        Self { nfa, dfa }
    }

    fn determinate(&mut self) {
        let conv_table = Map::default();
        let (start_closure, start_tag_table) = e_close([self.nfa.start_node()]);

        conv_table.insert(start_closure, self.dfa.node());

        for symbol in 0u8..=255 {}
    }
}
