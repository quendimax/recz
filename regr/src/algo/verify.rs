use crate::Epsilon;
use crate::algo::{self, VisitResult::*};
use crate::graph::Graph;
use crate::node::Node;
use redt::{SetU8, ops::*};

/// Checks if the given graph represents a valid DFA.
pub fn verify_dfa(graph: &Graph) -> bool {
    let mut is_dfa = true;
    algo::visit_nodes(graph.start_node(), |node| {
        if !verify_dfa_node(node) {
            is_dfa = false;
            return Stop;
        }
        Recurse
    });
    is_dfa
}

/// Checks if the given node meets the requirements of a DFA.
pub fn verify_dfa_node<'a>(node: Node<'a>) -> bool {
    let mut sym_mask = SetU8::empty();
    for (_, transitions) in node.targets() {
        for tr in transitions {
            if tr.contains(Epsilon) {
                return false;
            } else {
                if sym_mask.intersects(tr.as_set().as_ref()) {
                    return false;
                }
                sym_mask.include(tr.as_set().as_ref());
            }
        }
    }
    true
}
