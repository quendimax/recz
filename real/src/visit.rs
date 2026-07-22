use redt::Set;
use regr::{Node, Transition};

pub enum VisitResult {
    Stop,
    Continue,
    Recurse,
}

use VisitResult::*;

/// Recursively visit all nodes in breadth-first order in the graph starting
/// from the `start_node`, applying the given `action` to each node.
///
/// The `action` should return `true` if the node's children should be visited,
/// and `false` otherwise. So, `action` is called for `start_node` at least.
pub fn visit_nodes<'n, A>(start_node: Node<'n>, action: A)
where
    A: FnMut(Node<'n>) -> VisitResult,
{
    let mut action = action;
    let visited = Set::default();
    let mut unvisited = Vec::default();
    unvisited.push(start_node);
    while let Some(node) = unvisited.pop() {
        visited.insert(node);
        match action(node) {
            Stop => break,
            Continue => continue,
            Recurse => {
                for (_, target) in node.targets() {
                    if !visited.contains(&target) {
                        unvisited.push(target);
                    }
                }
            }
        }
    }
}

/// Recursively visit all transitions in breadth-first order in the graph
/// starting from the `start_node`, applying the given `action` to each
/// transition.
///
/// The `action` should return `true` if you want to visit transitions of the
/// current target node. Otherwise, the transitions will be skipped.
pub fn visit_transitions<'n, A>(start_node: Node<'n>, action: A)
where
    A: FnMut(Node<'n>, Transition<'n>, Node<'n>) -> VisitResult,
{
    let mut action = action;
    let visited = Set::default();
    let mut unvisited = Vec::new();
    unvisited.push(start_node);
    while let Some(node) = unvisited.pop() {
        visited.insert(node);
        for (tr, target) in node.targets() {
            match action(node, tr, target) {
                Stop => return,
                Continue => continue,
                Recurse => {
                    if !visited.contains(&target) {
                        unvisited.push(target);
                    }
                }
            }
        }
    }
}
