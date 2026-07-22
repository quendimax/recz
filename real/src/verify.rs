use crate::visit::VisitResult::*;
use redt::SetU8;
use regr::{Graph, Node};

/// Checks if the given graph represents a valid DFA.
pub fn verify_dfa(graph: &Graph) -> bool {
    let mut is_dfa = true;
    crate::visit_nodes(graph.start_node(), |node| {
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
    let out_symbols = SetU8::new();
    for (tr, _) in node.targets() {
        let tr_symbols = tr.symbols().into_set();
        if tr.is_epsilon() {
            return false;
        } else {
            if !out_symbols.is_disjoint(&tr_symbols) {
                return false;
            }
            out_symbols.insert_bytes(tr_symbols);
        }
    }
    true
}
