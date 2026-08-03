use crate::{Graph, Node, Tag};
use recz_adt::{Map, OrdSet, Set, SetU8};

pub fn determine(nfa: Graph) -> Graph {
    let dfa = Graph::new();
    let mut determ = Determinator::new(&nfa, &dfa);
    determ.determine();
    drop(determ);
    dfa
}

struct Determinator<'d, 'n> {
    nfa: &'n Graph,
    dfa: &'d Graph,
    has_final_nodes: bool,
    conv_table: Map<OrdSet<Node<'n>>, Node<'d>>,
    final_tags: Map<Node<'d>, Set<Tag>>,
    stack: Vec<(Node<'n>, Node<'n>)>,
}

struct EClosure<'n> {
    /// Epsilon closure of the nodes.
    nodes: OrdSet<Node<'n>>,

    /// All tags from this Espsilon closure.
    tags: Set<Tag>,

    /// The table that contains corresponding to every symbol a set of nodes
    /// that have outgoing edges with this symbols, and tags that are associated
    /// with those nodes.
    sym_table: Map<u8, (Set<Tag>, OrdSet<Node<'n>>)>,

    /// If any of `nodes` is a final node, this is `true`.
    is_final: bool,
}

impl<'d, 'n> Determinator<'d, 'n> {
    fn new(nfa: &'n Graph, dfa: &'d Graph) -> Self {
        assert!(
            !nfa.is(dfa),
            "NFA and DFA must be different graph instances"
        );
        assert!(dfa.is_empty(), "DFA must be empty");
        Self {
            nfa,
            dfa,
            has_final_nodes: false,
            conv_table: Map::default(),
            final_tags: Map::default(),
            stack: Vec::with_capacity(nfa.node_count()),
        }
    }

    fn determine(&mut self) {
        if self.nfa.is_empty() {
            return;
        }

        let start_closure = self.e_close([self.nfa.start_node()]);
        self.lambda(start_closure);

        if self.has_final_nodes {
            let epilogue_node = self.dfa.node();
            for node in self.dfa.nodes() {
                if node.is_final() {
                    let edge = node.connect(epilogue_node);
                    edge.add_tags(self.final_tags[&node].iter().copied());
                }
            }
            epilogue_node.epilogize();
        }

        // #[cfg(debug_assertions)]
        self.verify_dfa();
    }

    fn lambda(&mut self, closure: EClosure<'n>) -> Node<'d> {
        if let Some(dfa_node) = self.conv_table.get(&closure.nodes) {
            return *dfa_node;
        }

        let dfa_node = self.dfa.node();
        self.conv_table.insert(closure.nodes, dfa_node);
        if closure.is_final {
            self.has_final_nodes = true;
            dfa_node.finalize();
            let tags = self.final_tags.entry(dfa_node).or_default();
            tags.lazy_extend(closure.tags);
        }

        for (symbol, (tags, nodes)) in closure.sym_table {
            let sym_closure = self.e_close(nodes);
            let sym_dfa_node = self.lambda(sym_closure);
            let edge = dfa_node.connect(sym_dfa_node);
            edge.add_symbol(symbol);
            edge.add_tags(tags);
        }

        dfa_node
    }

    fn e_close(&mut self, nodes: impl Into<OrdSet<Node<'n>>>) -> EClosure<'n> {
        let nodes = nodes.into();
        let all_tags = Set::default();
        let tag_table = Map::<Node<'n>, Set<Tag>>::default();
        let sym_table = Map::<u8, (Set<Tag>, OrdSet<Node<'n>>)>::default();
        let mut is_final = false;

        for node in nodes {
            tag_table.insert(node, Set::default());
            for (edge, target) in node.targets() {
                if edge.is_epsilon() {
                    self.stack.push((target, node));
                } else {
                    for symbol in edge.symbols() {
                        let (_, closure) = sym_table.entry(symbol).or_default();
                        closure.insert(target);
                    }
                }
            }
            if node.is_final() {
                is_final = true;
            }
        }

        while let Some((node, source)) = self.stack.pop() {
            let edge = source.connect(node);
            let tags = tag_table.entry(node).or_default();
            tags.lazy_extend(edge.tags());
            tags.lazy_extend(tag_table[&source].iter().copied());
            all_tags.lazy_extend(edge.tags());

            for (edge, target) in node.targets() {
                if edge.is_epsilon() {
                    self.stack.push((target, node));
                } else {
                    for symbol in edge.symbols() {
                        let (sym_tags, closure) = sym_table.entry(symbol).or_default();
                        sym_tags.lazy_extend(tags.iter().copied());
                        closure.insert(target);
                    }
                }
            }
            if node.is_final() {
                is_final = true;
            }
        }
        EClosure {
            nodes: OrdSet::from_iter(tag_table.keys().copied()),
            tags: all_tags,
            sym_table,
            is_final,
        }
    }

    fn verify_dfa(&self) {
        for node in self.dfa.nodes() {
            let outgoing_symbols = SetU8::default();
            let mut has_epsilon = false;
            for (edge, target) in node.targets() {
                if edge.is_epsilon() {
                    if has_epsilon {
                        panic!("multiple outgoing epsilon edges are not allowed")
                    }
                    has_epsilon = true;
                    if !target.is_epilogue() {
                        panic!("only the epilogue node can be the target of epsilon edge")
                    }
                    if !node.is_final() {
                        panic!(
                            "only final nodes can have outgoing epsilon edges to the epilogue node"
                        )
                    }
                } else {
                    if outgoing_symbols.is_disjoint(&edge.symbols().into()) {
                        outgoing_symbols.insert_bytes(edge.symbols());
                    } else {
                        let overlap = outgoing_symbols.intersection(&edge.symbols().into());
                        panic!(
                            "DFA node has overlapping symbols: {} for nodes {} and {}",
                            overlap, node, target
                        );
                    }
                }
            }
        }
    }
}
