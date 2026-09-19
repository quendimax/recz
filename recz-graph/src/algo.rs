mod determ;
mod util;

use crate::{Graph, Node};

pub fn determine(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determ = determ::Determinator::new(nfa, &dfa);
    determ.determine();
    drop(determ);
    dfa
}

/// Looks for all paths starting from the given node and ending at the epilogue
/// node. Calls the handler for each path found.
pub fn find_paths(start_node: Node<'_>, handler: impl FnMut(&[Node<'_>])) {
    let mut pathfinder = util::PathFinder::new(start_node, handler);
    pathfinder.run();
}
