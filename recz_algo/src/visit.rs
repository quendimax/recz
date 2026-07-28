use recz_adt::Set;
use recz_graph::{Edge, Node};

/// The result of a visit action that is run within a [`visit_nodes`] or
/// [`visit_edges`] call.
///
/// Returning `VisitResult` action says to visit algorithm in the following way:
///
/// - `Stop` — stop visiting nodes immediately.
/// - `Continue` — don't visit children, but continue visiting siblings.
/// - `Recurse` — first visit children, then siblings.
pub enum VisitResult {
    /// Stop visiting nodes immediately.
    Stop,

    /// Don't visit children, but continue visiting siblings.
    Continue,

    /// First visit children, then siblings.
    Recurse,
}

use VisitResult::*;

/// Recursively visit all nodes in breadth-first order in the graph starting
/// from the `start_node`, applying the given `action` to each node.
///
/// The `action` should return `true` if the node's children should be visited,
/// and `false` otherwise. So, `action` is called for `start_node` at least.
pub fn visit_nodes<'n, A>(start_node: Node<'n>, mut action: A)
where
    A: FnMut(Node<'n>) -> VisitResult,
{
    let visited = Set::default();
    let mut unvisited = Vec::with_capacity(32);
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

/// Recursively visit all edges in breadth-first order in the graph starting
/// from the `start_node`, applying the given `action` to each edge.
///
/// The `action` should return `true` if you want to visit edges of the current
/// target node. Otherwise, the edges will be skipped.
pub fn visit_edges<'n, A>(start_node: Node<'n>, mut action: A)
where
    A: FnMut(Node<'n>, Edge<'n>, Node<'n>) -> VisitResult,
{
    let visited = Set::default();
    let mut unvisited = Vec::new();
    unvisited.push(start_node);
    while let Some(node) = unvisited.pop() {
        visited.insert(node);
        for (edge, target) in node.targets() {
            match action(node, edge, target) {
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
