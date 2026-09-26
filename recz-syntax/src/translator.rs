use crate::{ConcatHir, DisjunctHir, GroupHir, Hir, RepeatHir};
use recz_adt::{Set, SetU8};
use recz_graph::{Graph, Node, Tag};

/// Translator for translating a HIR into a NFA.
pub struct Translator<'a> {
    graph: &'a Graph,
}

impl<'a> Translator<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        // TODO: add optional checker for DFA graph
        Self { graph }
    }

    pub fn translate(&mut self, hir: &Hir, start_hode: Node<'a>) -> Node<'a> {
        self.translate_hir(hir, start_hode).node
    }

    fn translate_hir(&mut self, hir: &Hir, start_node: Node<'a>) -> Tail<'a> {
        match hir {
            Hir::Literal(literal) => self.translate_literal(literal, start_node),
            Hir::Class(class) => self.translate_class(class, start_node),
            Hir::Group(group) => self.translate_group(group, start_node),
            Hir::Repeat(repeat) => self.translate_repeat(repeat, start_node),
            Hir::Concat(concat) => self.translate_concat(concat, start_node),
            Hir::Disjunct(disjunct) => self.translate_disjunct(disjunct, start_node),
        }
    }

    fn translate_literal(&self, literal: &[u8], head_node: Node<'a>) -> Tail<'a> {
        if literal.is_empty() {
            let end_node = self.graph.node();
            head_node.connect(end_node);
            return Tail::new(end_node);
        }
        let mut curr = head_node;
        for byte in literal {
            let next = self.graph.node();
            curr.connect(next).add_symbol(*byte);
            curr = next;
        }
        Tail::new(curr)
    }

    fn translate_class(&self, class: &SetU8, head_node: Node<'a>) -> Tail<'a> {
        let end_node = self.graph.node();
        for range in class.ranges() {
            head_node.connect(end_node).add_symbols(range);
        }
        Tail::new(end_node)
    }

    /// Group.
    ///
    /// Only this function can create a new tag
    ///
    /// ```txt
    /// (○)──ε/+g0─→(○)...(○)──ε/-g0─→(○)
    /// ```
    fn translate_group(&mut self, group: &GroupHir, head_node: Node<'a>) -> Tail<'a> {
        let capture_group = self.graph.group(group.label());
        let open_tag = capture_group.open_tag();
        let close_tag = capture_group.close_tag();

        let first = self.graph.node();
        head_node.connect(first).add_tag(open_tag);

        let mut tail = self.translate_hir(group.inner(), first);
        tail.tags.insert(open_tag);
        tail.tags.insert(close_tag);

        let end_node = self.graph.node();
        tail.node.connect(end_node).add_tag(close_tag);
        tail.node = end_node;

        tail
    }

    fn translate_repeat(&mut self, repeat: &RepeatHir, head_node: Node<'a>) -> Tail<'a> {
        match repeat.multiplier().to_tuple() {
            (0, None) => self.translate_kleene_star(repeat.inner(), head_node),
            (1, None) => self.translate_one_plus(repeat.inner(), head_node),
            (n, None) => {
                let n_tail = self.translate_n_times(n - 1, repeat.inner(), head_node);
                let mut plus_tail = self.translate_one_plus(repeat.inner(), n_tail.node);
                plus_tail.tags.extend(n_tail.tags);
                plus_tail
            }
            (n, Some(m)) if n == m => self.translate_n_times(n, repeat.inner(), head_node),
            (n, Some(m)) if n < m => {
                let n_tail = self.translate_n_times(n, repeat.inner(), head_node);
                let mut tail = self.translate_possible(m - n, repeat.inner(), n_tail.node);
                tail.tags.extend(n_tail.tags);
                tail
            }
            (n, Some(m)) => {
                panic!("invalid repetition counters: {{{n},{m}}}");
            }
        }
    }

    /// Kleene star `a*`.
    ///
    /// ```txt
    ///          ╭────ε────╮
    ///          ↓         │
    /// (1)──ε─→(2)──'a'─→(3)──ε─→(4)
    ///  │                         ↑
    ///  ╰────────────ε────────────╯
    /// ```
    fn translate_kleene_star(&mut self, inner_hir: &Hir, head_node: Node<'a>) -> Tail<'a> {
        let first = self.graph.node();
        head_node.connect(first);
        let mut tail = self.translate_hir(inner_hir, first);
        let end = self.graph.node();
        tail.node.connect(first);
        tail.node.connect(end);
        head_node.connect(end);
        tail.node = end;
        tail
    }

    /// Plus iteration `a+`.
    ///
    /// ```txt
    ///          ╭────ε────╮
    ///          ↓         │
    /// (1)──ε─→(2)──'a'─→(3)──ε─→(4)
    /// ```
    fn translate_one_plus(&mut self, inner_hir: &Hir, head_node: Node<'a>) -> Tail<'a> {
        let first = self.graph.node();
        head_node.connect(first);
        let mut tail = self.translate_hir(inner_hir, first);
        let end = self.graph.node();
        tail.node.connect(first);
        tail.node.connect(end);
        tail.node = end;
        tail
    }

    /// N-times iteration `a{n}`.
    ///
    /// ```txt
    /// (1)──'a'─→(2)──'a'─→(3)──'a'─→(4)
    /// ```
    fn translate_n_times(&mut self, n: usize, inner_hir: &Hir, head_node: Node<'a>) -> Tail<'a> {
        if n == 0 {
            let end_node = self.graph.node();
            head_node.connect(end_node);
            return Tail::new(end_node);
        }
        let mut tags = Set::default();
        let mut first = head_node;
        for _ in 0..n {
            let tail = self.translate_hir(inner_hir, first);
            tags.extend(tail.tags);
            first = tail.node;
        }
        Tail { node: first, tags }
    }

    /// Possible repetition `a{0,n}`.
    ///
    /// ```txt
    /// (○)─ε─→(○)─'a'→(○)─ε─→(○)─ε─→(○)─'a'─→(○)─ε─→(○)─...─ε─→(○)
    ///  │                     │                      │          ↑
    ///  │                     │                      ╰─────ε────╯
    ///  │                     ╰────────────────ε────────────────╯
    ///  ╰───────────────────────────ε───────────────────────────╯
    /// ```
    fn translate_possible(&mut self, n: usize, inner_hir: &Hir, head_node: Node<'a>) -> Tail<'a> {
        let mut tags = Set::default();
        let mut head_nodes = Vec::with_capacity(n);
        let mut curr = head_node;
        for _ in 0..n {
            head_nodes.push(curr);
            let before = self.graph.node();
            curr.connect(before);
            let tail = self.translate_hir(inner_hir, before);
            tags.extend(tail.tags);
            let after = self.graph.node();
            tail.node.connect(after);
            curr = after;
        }
        for head in head_nodes {
            head.connect(curr);
        }
        Tail { node: curr, tags }
    }

    fn translate_concat(&mut self, concat: &ConcatHir, head_node: Node<'a>) -> Tail<'a> {
        let items = concat.items();
        if items.is_empty() {
            let end_node = self.graph.node();
            head_node.connect(end_node);
            return Tail::new(end_node);
        }
        let mut tags = Set::default();
        let mut first = head_node;
        for hir in &items[..items.len() - 1] {
            let tail = self.translate_hir(hir, first);
            tags.extend(tail.tags);
            first = tail.node;
        }
        let hir = items.last().unwrap();
        let tail = self.translate_hir(hir, first);
        tags.extend(tail.tags);
        Tail {
            node: tail.node,
            tags,
        }
    }

    /// ```txt
    ///  ╭───ε──→(○)──'a'─→(○)──ε───╮
    ///  │                          ↓
    /// (○)──ε──→(○)──'b'─→(○)──ε─→(○)
    ///  │                          ↑
    ///  ╰───ε──→(○)──'c'─→(○)──ε───╯
    /// ```
    fn translate_disjunct(&mut self, disjunct: &DisjunctHir, head_node: Node<'a>) -> Tail<'a> {
        let mut branch_tails = Vec::new();
        for hir in disjunct.alternatives() {
            let first = self.graph.node();
            head_node.connect(first);
            let tail = self.translate_hir(hir, first);
            branch_tails.push(tail);
        }
        let tail = Tail::new(self.graph.node());
        for lhs_tail in &branch_tails {
            let edge = lhs_tail.node.connect(tail.node);
            for rhs_tail in &branch_tails {
                if lhs_tail.node != rhs_tail.node {
                    for tag in &rhs_tail.tags {
                        if let Some(tag) = tag.delete_group() {
                            edge.add_tag(tag);
                        }
                    }
                }
            }
        }
        tail
    }
}

#[derive(Debug)]
struct Tail<'a> {
    node: Node<'a>,
    tags: Set<Tag>,
}

impl<'a> Tail<'a> {
    fn new(node: Node<'a>) -> Self {
        Self {
            node,
            tags: Set::default(),
        }
    }
}

#[cfg(test)]
#[path = "utest/translator.rs"]
mod utest;
