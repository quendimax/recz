use recz_adt::{Map, OrdSet, Set};
use recz_graph::{Graph, Node, Tag};

pub fn determinate(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determ = Determinator::new(nfa, &dfa);
    determ.determinate();
    drop(determ);
    dfa
}

struct Determinator<'d, 'n> {
    nfa: &'n Graph,
    dfa: &'d Graph,
    conv_table: Map<OrdSet<Node<'n>>, Node<'d>>,
}

impl<'d, 'n> Determinator<'d, 'n> {
    fn new(nfa: &'n Graph, dfa: &'d Graph) -> Self {
        assert_ne!(nfa.gid(), dfa.gid());
        assert!(dfa.is_empty(), "DFA must be empty");
        Self {
            nfa,
            dfa,
            conv_table: Map::default(),
        }
    }

    fn determinate(&mut self) {
        let start_closure = e_close([self.nfa.start_node()]);
        self.lambda(start_closure);
    }

    fn lambda(&self, closure: Closure<'n>) -> Node<'d> {
        if let Some(dfa_node) = self.conv_table.get(&closure.nodes) {
            return *dfa_node;
        }

        let dfa_node = self.dfa.node();
        self.conv_table.insert(closure.nodes, dfa_node);
        if closure.is_final {
            dfa_node.finalize();
        }

        for (symbol, (tags, nodes)) in closure.sym_table {
            let sym_closure = e_close(nodes);
            let sym_dfa_node = self.lambda(sym_closure);
            let edge = dfa_node.connect(sym_dfa_node);
            edge.add_symbol(symbol);
            tags.iter().for_each(|tag| edge.add_tag(*tag));
        }

        dfa_node
    }
}

struct Closure<'a> {
    nodes: OrdSet<Node<'a>>,
    sym_table: Map<u8, (Set<Tag>, OrdSet<Node<'a>>)>,
    is_final: bool,
}

fn e_close<'a>(nodes: impl Into<OrdSet<Node<'a>>>) -> Closure<'a> {
    let nodes = nodes.into();
    let mut stack = Vec::new();
    let tag_table = Map::<Node<'a>, Set<Tag>>::default();
    let sym_table = Map::<u8, (Set<Tag>, OrdSet<Node<'a>>)>::default();
    let mut is_final = false;

    for node in nodes {
        tag_table.insert(node, Set::default());
        for (edge, target) in node.targets() {
            if edge.is_epsilon() {
                stack.push((target, node));
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

    while let Some((node, source)) = stack.pop() {
        let tags = tag_table.entry(node).or_default();
        tags.lazy_extend(source.connect(node).tags());
        tags.lazy_extend(tag_table[&source].iter().copied());

        for (edge, target) in node.targets() {
            if edge.is_epsilon() {
                stack.push((target, node));
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
    Closure {
        nodes: OrdSet::from_iter(tag_table.keys().copied()),
        sym_table,
        is_final,
    }
}
