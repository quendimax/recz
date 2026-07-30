use recz_adt::{Map, OrdSet, Set};
use recz_graph::{Node, Tag};

/// The result of a visit action that is run within a [`visit_nodes`] or
/// [`visit_edges`] call.
///
/// Returning `VisitWay` action says to visit algorithm in the following way:
///
/// - `Stop` — stop visiting nodes immediately.
/// - `Sideways` — don't visit children of current node, but continue visiting
///   siblings.
/// - `Descend` — first visit children, then siblings.
pub enum VisitWay {
    /// Stop visiting nodes immediately.
    Stop,

    /// Skip children of the current node, and continue visiting siblings.
    Sideways,

    /// First visit children, then siblings.
    Descend,
}

use VisitWay::*;

/// Recursively visit all nodes in depth-first order in the graph starting from
/// the `start_node`, applying the given `action` to each node.
///
/// The `action` should return `true` if the node's children should be visited,
/// and `false` otherwise. So, `action` is called for `start_node` at least.
pub fn visit_nodes<'a, A>(start_node: Node<'a>, mut action: A)
where
    A: FnMut(Node<'a>, Option<Node<'a>>) -> VisitWay,
{
    let visited = Set::default();
    let mut unvisited = Vec::with_capacity(32);
    unvisited.push((start_node, None));
    while let Some((node, source)) = unvisited.pop() {
        visited.insert(node);
        match action(node, source) {
            Stop => break,
            Sideways => continue,
            Descend => {
                for (_, target) in node.targets() {
                    if !visited.contains(&target) {
                        unvisited.push((target, Some(node)));
                    }
                }
            }
        }
    }
}

/// Returns the epsilon closure of a set of nodes.
///
/// The epsilon closure is a map from each node to the set of tags that are
/// reachable from the start nodes via epsilon edges.
///
/// Returns a tuple of the closure set and the tag table.
pub fn e_close<'a>(
    nodes: impl IntoIterator<Item = Node<'a>>,
) -> (OrdSet<Node<'a>>, Map<Node<'a>, Set<Tag>>) {
    let mut stack = Vec::new();
    let tag_table = Map::<Node<'a>, Set<Tag>>::default();
    for node in nodes {
        for (edge, target) in node.targets() {
            if edge.is_epsilon() {
                stack.push((target, node));
            }
        }
        tag_table.insert(node, Set::default());
    }

    while let Some((node, source)) = stack.pop() {
        let tags = tag_table.entry(node).or_default();
        let edge = source.connect(node);
        tags.lazy_extend(edge.tags());
        tags.lazy_extend(tag_table[&source].iter().copied());

        for (edge, target) in node.targets() {
            if edge.is_epsilon() {
                stack.push((target, node));
            }
        }
    }
    let closure = OrdSet::from_iter(tag_table.keys().copied());
    (closure, tag_table)
}
