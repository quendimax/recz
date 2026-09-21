mod determ;
mod pathfd;
mod tagclean;
mod util;

use crate::{Edge, Graph, Node};

pub fn determine(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determ = determ::Determinator::new(nfa, &dfa);
    determ.determine();
    drop(determ);
    dfa
}

/// Looks for all paths starting from the `start_node` and ending at the
/// `end_node`. Calls the handler for each path found.
pub fn find_paths(start_node: Node<'_>, end_node: Node<'_>, handler: impl FnMut(&[Edge<'_>])) {
    let mut pathfinder = pathfd::PathFinder::new(start_node, end_node, handler);
    pathfinder.run();
}

pub fn clean_tags_up(dfa: &Graph) -> Graph {
    let new_dfa = Graph::new();
    let mut tagmin = tagclean::TagCleaner::new(dfa, &new_dfa);
    tagmin.run();
    drop(tagmin);
    new_dfa
}
