use crate::{Graph, Node, Tag};
use recz_adt::{Map, OrdSet, Set};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;

pub fn determine(nfa: &Graph) -> Graph {
    let dfa = Graph::new();
    let mut determ = Determinator::new(nfa, &dfa);
    determ.determine();
    drop(determ);
    dfa
}

struct Determinator<'d, 'n> {
    nfa: &'n Graph,
    dfa: &'d Graph,
    conv_table: Map<Rc<OrdSet<Node<'n>>>, Node<'d>>,
    closure_eval: EClosureEval<'n>,
    final_tags: Map<Node<'d>, Vec<Tag>>,
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
            conv_table: Map::default(),
            closure_eval: EClosureEval::new(),
            final_tags: Map::default(),
        }
    }

    fn determine(&mut self) {
        if self.nfa.is_empty() {
            return;
        }

        for group in self.nfa.groups() {
            self.dfa.group(group.label().clone());
        }

        let start_closure = self
            .closure_eval
            .eval(Rc::new([self.nfa.start_node()].into()));

        self.recurse(start_closure);

        if !self.final_tags.is_empty() {
            let epilogue_node = self.dfa.node();
            for node in self.dfa.nodes() {
                if node.is_final() {
                    let edge = node.connect(epilogue_node);
                    edge.add_tags(self.final_tags[&node].iter().copied());
                }
            }
            epilogue_node.epilogize();
        }
    }

    fn recurse(&mut self, closure: Rc<EClosure<'n>>) -> Node<'d> {
        let dfa_node = self.dfa.node();
        self.conv_table.insert(Rc::clone(&closure.nodes), dfa_node);

        if let Some(clo_final_tags) = &closure.final_tags {
            dfa_node.finalize();
            let mut final_tags = Vec::with_capacity(clo_final_tags.len());
            final_tags.extend(clo_final_tags.iter().copied());
            self.final_tags.insert(dfa_node, final_tags);
        }

        for (symbol, (tags, nodes)) in &closure.sym_table {
            let sym_closure = self.closure_eval.eval(Rc::clone(nodes));
            let sym_dfa_node = self
                .conv_table
                .get(&sym_closure.nodes)
                .copied()
                .unwrap_or_else(|| self.recurse(sym_closure));
            let edge = dfa_node.connect(sym_dfa_node);
            edge.add_symbol(*symbol);
            edge.add_tags(tags.iter().copied());
        }

        dfa_node
    }
}

#[derive(Debug)]
struct EClosure<'n> {
    /// Epsilon closure of the nodes.
    nodes: Rc<OrdSet<Node<'n>>>,

    /// The table that contains corresponding to every symbol a set of nodes
    /// that have outgoing edges with this symbols, and tags that are associated
    /// with those nodes.
    sym_table: Map<u8, (Set<Tag>, Rc<OrdSet<Node<'n>>>)>,

    /// All tags that are going to the final nodes. If it is `None` the closure
    /// doesn't have any final nodes.
    final_tags: Option<Set<Tag>>,
}

impl<'n> EClosure<'n> {
    pub fn default() -> Rc<Self> {
        Rc::new(Self {
            nodes: Rc::new(OrdSet::default()),
            sym_table: Map::default(),
            final_tags: None,
        })
    }
}

#[derive(Debug)]
struct EClosureEval<'n> {
    cache: Map<Rc<OrdSet<Node<'n>>>, Rc<EClosure<'n>>>,
    visited: Map<Node<'n>, u64>,
    tag_collector: TagCollector,
    closure: Rc<EClosure<'n>>,
}

impl<'n> EClosureEval<'n> {
    fn new() -> Self {
        Self {
            cache: Map::default(),
            visited: Map::default(),
            tag_collector: TagCollector::new(),
            closure: EClosure::default(),
        }
    }

    fn eval(&mut self, start_nodes: Rc<OrdSet<Node<'n>>>) -> Rc<EClosure<'n>> {
        if let Some(closure) = self.cache.get(&start_nodes) {
            return Rc::clone(closure);
        }

        self.tag_collector.clear();
        assert!(self.visited.is_empty());

        for start_node in start_nodes.iter().copied() {
            self.visited.clear();
            self.tag_collector.clear();
            self.eval_one(start_node);
        }

        self.cache.insert(start_nodes, Rc::clone(&self.closure));
        let mut return_closure = EClosure::default();
        std::mem::swap(&mut self.closure, &mut return_closure);
        // dbg!(&return_closure);
        return_closure
    }

    fn eval_one(&mut self, node: Node<'n>) {
        let new_check = self.tag_collector.checksum();
        if self.visited.replace(node, new_check) == Some(new_check) {
            return;
        }
        self.closure.nodes.insert(node);

        if node.is_final() {
            let closure = Rc::get_mut(&mut self.closure).unwrap();
            if let Some(tags) = &mut closure.final_tags {
                merge_tags(tags, &self.tag_collector);
            } else {
                closure.final_tags = Some(Set::from_iter(self.tag_collector.tags()));
            }
        }

        for (edge, target) in node.targets() {
            self.tag_collector.extend(edge.tags());

            if edge.is_epsilon() {
                self.eval_one(target);
            } else {
                let closure = Rc::get_mut(&mut self.closure).unwrap();
                for symbol in edge.symbols() {
                    if let Some((tags, sym_closure)) = closure.sym_table.get_mut(&symbol) {
                        merge_tags(tags, &self.tag_collector);
                        sym_closure.insert(target);
                    } else {
                        closure.sym_table.insert(
                            symbol,
                            (
                                Set::from_iter(self.tag_collector.tags()),
                                Rc::new(OrdSet::from([target])),
                            ),
                        );
                    }
                }
            }

            self.tag_collector.shorten(edge.tags().rev());
        }

        self.visited.remove(&node);
    }
}

/// The Grail of this determinization algorithm. It decides how tags are merged.
fn merge_tags(dest_tags: &mut Set<Tag>, source_tags: &TagCollector) {
    // for tag in src.into_iter() {
    //     match tag {
    //         Tag::OpenGroup(_) | Tag::CloseGroup(_) | Tag::DeleteGroup(_) => {
    //             dest.insert(tag);
    //         }
    //     }
    // }
    dest_tags.extend(source_tags.tags());
}

#[derive(Debug)]
struct TagCollector {
    tags: Vec<Tag>,
    hashers: Vec<DefaultHasher>,
}

impl TagCollector {
    fn new() -> Self {
        let mut initial_hasher = DefaultHasher::new();
        initial_hasher.write_u64(0xDEADBEEF);
        Self {
            tags: Vec::default(),
            hashers: vec![initial_hasher],
        }
    }

    fn clear(&mut self) {
        self.tags.clear();
        self.hashers.clear();
        let mut initial_hasher = DefaultHasher::new();
        initial_hasher.write_u64(0xDEADBEEF);
        self.hashers.push(initial_hasher);
    }

    fn insert(&mut self, tag: Tag) {
        if !self.tags.contains(&tag) {
            let mut hasher = self.hashers.last().unwrap().clone();
            tag.hash(&mut hasher);
            self.hashers.push(hasher);
        }
        self.tags.push(tag);
    }

    fn extend(&mut self, tags: impl IntoIterator<Item = Tag>) {
        for tag in tags.into_iter() {
            self.insert(tag);
        }
    }

    fn remove(&mut self, tag: Tag) {
        assert_eq!(self.tags.pop(), Some(tag));
        if !self.tags.contains(&tag) {
            self.hashers.pop();
        }
    }

    fn shorten(&mut self, tags: impl IntoIterator<Item = Tag>) {
        for tag in tags.into_iter() {
            self.remove(tag);
        }
    }

    fn contains(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }

    fn tags(&self) -> impl Iterator<Item = Tag> + '_ {
        self.tags.iter().copied()
    }

    fn checksum(&self) -> u64 {
        self.hashers.last().unwrap().finish()
    }
}
