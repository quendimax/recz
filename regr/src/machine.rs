use crate::graph::Graph;
use crate::node::Node;
use crate::transition::Transition;
use redt::Set;

pub enum VisitResult {
    Stop,
    Continue,
    Recurse,
}

use VisitResult::*;

/// Algorithm machine evaluates different algorithms for a specified finite
/// automaton graph.
///
/// No one of these algorithms don't modify the graph, only analyze it, and
/// create new ones.
pub struct Machine<'a> {
    graph: &'a Graph,
}

impl<'a> Machine<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }

    /// Recursively visit all nodes in breadth-first order in the graph starting
    /// from the `start_node`, applying the given `action` to each node.
    ///
    /// The `action` should return [`Recurse`] if the node's children should be
    /// visited, [`Continue`] if only siblings should be visiterd, or [`Stop`]
    /// if you want to stop the visiting. So, `action` is called for
    /// `start_node` at least.
    pub fn visit_nodes<'n, A>(&self, start_node: Node<'n>, action: A)
    where
        A: FnMut(Node<'n>) -> VisitResult,
    {
        debug_assert_eq!(start_node.gid(), self.graph.gid());

        let mut action = action;
        // preallocate maximum possible size of stack
        let mut unvisited = Vec::with_capacity(self.graph.len());
        let mut visited = Set::with_capacity(self.graph.len());
        unvisited.push(start_node);
        while let Some(node) = unvisited.pop() {
            visited.insert(node);
            match action(node) {
                Stop => break,
                Continue => continue,
                Recurse => {
                    for (target, _) in node.targets() {
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
    /// The `action` should return [`Recurse`] if the node's children should be
    /// visited, [`Continue`] if only siblings should be visiterd, or [`Stop`]
    /// if you want to stop the visiting. So, `action` is called for
    /// `start_node` at least.
    pub fn visit_transitions<'n, A>(self, start_node: Node<'n>, action: A)
    where
        A: FnMut(Node<'n>, Transition<'n>, Node<'n>) -> VisitResult,
    {
        debug_assert_eq!(start_node.gid(), self.graph.gid());

        let mut action = action;
        // preallocate maximum possible size of stack
        let mut unvisited = Vec::with_capacity(self.graph.len());
        let mut visited = Set::with_capacity(self.graph.len());
        unvisited.push(start_node);
        while let Some(node) = unvisited.pop() {
            visited.insert(node);
            for (target, transitions) in node.targets() {
                for tr in transitions {
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
    }
}
