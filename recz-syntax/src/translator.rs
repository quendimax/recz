use crate::{ConcatHir, DisjunctHir, GroupHir, Hir, RepeatHir};
use recz_adt::{Set, SetU8};
use recz_graph::{Graph, Node, Tag};

struct Pair<'a> {
    first: Node<'a>,
    last: Node<'a>,
}

fn pair<'a>(first: Node<'a>, last: Node<'a>) -> Pair<'a> {
    Pair { first, last }
}

/// Translator for translating a HIR into a NFA.
pub struct Translator<'a> {
    graph: &'a Graph,
}

impl<'a> Translator<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        // TODO: add optional checker for DFA graph
        Self { graph }
    }

    pub fn translate(&mut self, hir: &Hir, start_hode: Node<'a>, end_node: Node<'a>) {
        _ = self.translate_hir(hir, pair(start_hode, end_node));
    }

    fn translate_hir(&mut self, hir: &Hir, sub: Pair<'a>) -> Set<Tag> {
        match hir {
            Hir::Literal(literal) => self.translate_literal(literal, sub),
            Hir::Class(class) => self.translate_class(class, sub),
            Hir::Group(group) => self.translate_group(group, sub),
            Hir::Repeat(repeat) => self.translate_repeat(repeat, sub),
            Hir::Concat(concat) => self.translate_concat(concat, sub),
            Hir::Disjunct(disjunct) => self.translate_disjunct(disjunct, sub),
        }
    }

    fn translate_literal(&self, literal: &[u8], sub: Pair<'a>) -> Set<Tag> {
        if literal.is_empty() {
            sub.first.connect(sub.last);
            return Set::default();
        }
        let mut first = sub.first;
        for byte in &literal[..literal.len() - 1] {
            let next = self.graph.node();
            first.connect(next).add_symbol(*byte);
            first = next;
        }
        let last_byte = literal.last().unwrap();
        first.connect(sub.last).add_symbol(*last_byte);
        Set::default()
    }

    fn translate_class(&self, class: &SetU8, sub: Pair<'a>) -> Set<Tag> {
        for range in class.ranges() {
            sub.first.connect(sub.last).add_symbols(range);
        }
        Set::default()
    }

    // Only this function can create a new tag
    //
    // (○)──ε/+g0─→(○)...(○)──ε/-g0─→(○)
    //
    fn translate_group(&mut self, group: &GroupHir, sub: Pair<'a>) -> Set<Tag> {
        let capture_group = self.graph.group(group.label());
        let open_tag = capture_group.open_tag();
        let close_tag = capture_group.close_tag();

        let first = self.graph.node();
        sub.first.connect(first).add_tag(open_tag);

        let last = self.graph.node();
        last.connect(sub.last).add_tag(close_tag);

        let tags = self.translate_hir(group.inner(), pair(first, last));
        tags.insert(open_tag);
        tags.insert(close_tag);
        tags
    }

    fn translate_repeat(&mut self, repeat: &RepeatHir, mut sub: Pair<'a>) -> Set<Tag> {
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
                let last = self.graph.node();
                sub.first.connect(first);
                last.connect(sub.last);
                last.connect(first);
                sub.first.connect(sub.last);
                self.translate_hir(repeat.inner(), pair(first, last))
            }
            //
            //          ╭────ε────╮
            //          ↓         │
            // (1)──ε─→(2)──'a'─→(3)──ε─→(4)
            //
            (1, None) => {
                let first = self.graph.node();
                let last = self.graph.node();
                sub.first.connect(first);
                last.connect(sub.last);
                last.connect(first);
                self.translate_hir(repeat.inner(), pair(first, last))
            }
            //
            //                               ╭─────ε─────╮
            //                               ↓           │
            // (1)──'a'──...──'a'─→(n)──ε─→(n+1)──'a'─→(n+2)──ε─→(n+3)
            //
            (n, None) => {
                let mut tags = Set::default();
                let mut first = sub.first;
                for _ in 1..n {
                    let last = self.graph.node();
                    let new_tags = self.translate_hir(repeat.inner(), pair(first, last));
                    tags.extend(new_tags);
                    first = last;
                }
                sub.first = first;
                let first = self.graph.node();
                let last = self.graph.node();
                let new_tags = self.translate_hir(repeat.inner(), pair(first, last));
                tags.extend(new_tags);
                sub.first.connect(first);
                last.connect(sub.last);
                last.connect(first);
                tags
            }
            //
            // (0)──'a'──(1)──'a'──...──'a'─→(n)
            //
            (n, Some(m)) if n == m => {
                let mut tags = Set::default();
                if n == 0 {
                    sub.first.connect(sub.last);
                    tags
                } else {
                    let mut first = sub.first;
                    for _ in 0..n - 1 {
                        let last = self.graph.node();
                        let new_tags = self.translate_hir(repeat.inner(), pair(first, last));
                        tags.extend(new_tags);
                        first = last;
                    }
                    let new_tags = self.translate_hir(repeat.inner(), pair(first, sub.last));
                    tags.extend(new_tags);
                    tags
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
                let mut first = sub.first;
                for _ in 0..n {
                    let last = self.graph.node();
                    let new_tags = self.translate_hir(repeat.inner(), pair(first, last));
                    tags.extend(new_tags);
                    first = last;
                }
                for _ in n..m {
                    let mid_one = self.graph.node();
                    first.connect(mid_one);
                    let mid_two = self.graph.node();
                    let new_tags = self.translate_hir(repeat.inner(), pair(mid_one, mid_two));
                    tags.extend(new_tags);
                    let last = self.graph.node();
                    mid_two.connect(last);
                    first.connect(sub.last);
                    first = last;
                }
                first.connect(sub.last);
                tags
            }
            (n, Some(m)) => {
                panic!("invalid repetition counters: {{{n},{m}}}");
            }
        }
    }

    fn translate_concat(&mut self, concat: &ConcatHir, sub: Pair<'a>) -> Set<Tag> {
        let mut tags = Set::default();
        let items = concat.items();
        if items.is_empty() {
            sub.first.connect(sub.last);
            return tags;
        }
        let mut first = sub.first;
        for hir in &items[..items.len() - 1] {
            let last = self.graph.node();
            let new_tags = self.translate_hir(hir, pair(first, last));
            tags.extend(new_tags);
            first = last;
        }
        let hir = items.last().unwrap();
        let new_tags = self.translate_hir(hir, pair(first, sub.last));
        tags.extend(new_tags);
        tags
    }

    /// ```txt
    ///  ╭───ε──→(○)──'a'─→(○)──ε───╮
    ///  │                          ↓
    /// (○)──ε──→(○)──'b'─→(○)──ε─→(○)
    ///  │                          ↑
    ///  ╰───ε──→(○)──'c'─→(○)──ε───╯
    /// ```
    fn translate_disjunct(&mut self, disjunct: &DisjunctHir, sub: Pair<'a>) -> Set<Tag> {
        let mut branches = Vec::new();
        for hir in disjunct.alternatives() {
            let first = self.graph.node();
            let last = self.graph.node();
            sub.first.connect(first);
            let branch_tags = self.translate_hir(hir, pair(first, last));
            branches.push((last, branch_tags));
        }
        for (last, _) in &branches {
            let mut is_connected = false;
            for (other_last, other_tags) in &branches {
                if last != other_last {
                    for tag in other_tags {
                        if let Some(tag) = tag.delete_group() {
                            last.connect(sub.last).add_tag(tag);
                            is_connected = true;
                        }
                    }
                }
            }
            if !is_connected {
                last.connect(sub.last);
            }
        }
        let mut tags = Set::default();
        for (_, branch_tags) in &branches {
            tags.extend(branch_tags);
        }
        tags
    }
}

#[cfg(test)]
#[path = "utest/translator.rs"]
mod utest;
