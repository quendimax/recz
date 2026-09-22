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

    // Only this function can create a new tag
    //
    // (○)──ε/+g0─→(○)...(○)──ε/-g0─→(○)
    //
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
        match repeat.iter_hint() {
            // Kleene star
            //          ╭────ε────╮
            //          ↓         │
            // (1)──ε─→(2)──'a'─→(3)──ε─→(4)
            //  │                         ↑
            //  ╰────────────ε────────────╯
            //
            (0, None) => {
                let first = self.graph.node();
                head_node.connect(first);
                let mut tail = self.translate_hir(repeat.inner(), first);
                let end = self.graph.node();
                tail.node.connect(first);
                tail.node.connect(end);
                head_node.connect(end);
                tail.node = end;
                tail
            }
            //
            //          ╭────ε────╮
            //          ↓         │
            // (1)──ε─→(2)──'a'─→(3)──ε─→(4)
            //
            (1, None) => {
                let first = self.graph.node();
                head_node.connect(first);
                let mut tail = self.translate_hir(repeat.inner(), first);
                let end = self.graph.node();
                tail.node.connect(first);
                tail.node.connect(end);
                tail.node = end;
                tail
            }
            //
            //                               ╭─────ε─────╮
            //                               ↓           │
            // (1)──'a'──...──'a'─→(n)──ε─→(n+1)──'a'─→(n+2)──ε─→(n+3)
            //
            (n, None) => {
                let mut tags = Set::default();
                let mut first = head_node;
                for _ in 1..n {
                    let tail = self.translate_hir(repeat.inner(), first);
                    tags.extend(tail.tags);
                    first = tail.node;
                }
                let tmp = first;
                let first = self.graph.node();
                tmp.connect(first);
                let mut tail = self.translate_hir(repeat.inner(), first);
                tags.extend(tail.tags);
                let end = self.graph.node();
                tail.node.connect(first);
                tail.node.connect(end);
                tail.node = end;
                tail.tags = tags;
                tail
            }
            //
            // (0)──'a'──(1)──'a'──...──'a'─→(n)
            //
            (n, Some(m)) if n == m => {
                let mut tags = Set::default();
                if n == 0 {
                    let end_node = self.graph.node();
                    head_node.connect(end_node);
                    Tail::new(end_node)
                } else {
                    let mut first = head_node;
                    for _ in 0..n - 1 {
                        let tail = self.translate_hir(repeat.inner(), first);
                        tags.extend(tail.tags);
                        first = tail.node;
                    }
                    let mut tail = self.translate_hir(repeat.inner(), first);
                    tail.tags.extend(tags);
                    tail
                }
            }
            //
            // (0)──'a'─..─'a'─→(n)──ε─→(○)──'a'─→(○)──ε─→(○)──ε─→(○)──'a'──(○)──ε─→(○)──...──ε─→(○)
            //                   │                         │                         │            ↑
            //                   │                         │                         ╰──────ε─────╯
            //                   │                         ╰───────────────────ε──────────────────╯
            //                   ╰────────────────────────────────ε───────────────────────────────╯
            //
            (n, Some(m)) if n < m => {
                let mut tags = Set::default();
                let mut curr = head_node;
                for _ in 0..n {
                    let tail = self.translate_hir(repeat.inner(), curr);
                    tags.extend(tail.tags);
                    curr = tail.node;
                }
                let mut last_nodes = Vec::with_capacity(m - n);
                for _ in n..m {
                    last_nodes.push(curr);
                    let mid_one = self.graph.node();
                    curr.connect(mid_one);
                    let tail = self.translate_hir(repeat.inner(), mid_one);
                    tags.extend(tail.tags);
                    let last = self.graph.node();
                    tail.node.connect(last);
                    curr = last;
                }
                for last in last_nodes {
                    last.connect(curr);
                }
                Tail { node: curr, tags }
            }
            (n, Some(m)) => {
                panic!("invalid repetition counters: {{{n},{m}}}");
            }
        }
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
