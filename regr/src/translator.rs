use crate::graph::Graph;
use crate::isa::Inst::*;
use crate::node::Node;
use redt::SetU8;
use resy::{ConcatHir, DisjunctHir, GroupHir, Hir, RepeatHir};
use std::cell::Cell;

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
    next_reg: Cell<u32>,
}

impl<'a> Translator<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        // TODO: add optional checker for DFA graph
        Self {
            graph,
            next_reg: Cell::new(0),
        }
    }

    pub fn translate(&mut self, hir: &Hir, start_hode: Node<'a>, end_node: Node<'a>) {
        self.translate_hir(hir, pair(start_hode, end_node));
    }

    fn translate_hir(&mut self, hir: &Hir, sub: Pair<'a>) {
        match hir {
            Hir::Literal(literal) => self.translate_literal(literal, sub),
            Hir::Class(class) => self.translate_class(class, sub),
            Hir::Group(group) => self.translate_group(group, sub),
            Hir::Repeat(repeat) => self.translate_repeat(repeat, sub),
            Hir::Concat(concat) => self.translate_concat(concat, sub),
            Hir::Disjunct(disjunct) => self.translate_disjunct(disjunct, sub),
        }
    }

    fn translate_literal(&self, literal: &[u8], sub: Pair<'a>) {
        if literal.is_empty() {
            sub.first.connect(sub.last, Nop);
            return;
        }
        let mut first = sub.first;
        for byte in &literal[..literal.len() - 1] {
            let next = self.graph.node();
            first.connect(next, Nop).merge(*byte);
            first = next;
        }
        let last_byte = literal.last().unwrap();
        first.connect(sub.last, Nop).merge(*last_byte);
    }

    fn translate_class(&self, class: &SetU8, sub: Pair<'a>) {
        for range in class.ranges() {
            sub.first.connect(sub.last, Nop).merge(range);
        }
    }

    // Only this function can create a new tag
    fn translate_group(&mut self, group: &GroupHir, sub: Pair<'a>) {
        let first = self.graph.node();
        sub.first.connect(first, Nop);

        let last = self.graph.node();
        last.connect(sub.last, Nop);

        self.translate_hir(group.inner(), pair(first, last));
    }

    fn translate_repeat(&mut self, repeat: &RepeatHir, mut sub: Pair<'a>) {
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
                sub.first.connect(first, Nop);
                last.connect(sub.last, Nop);
                last.connect(first, Nop);
                sub.first.connect(sub.last, Nop);
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
                sub.first.connect(first, Nop);
                last.connect(sub.last, Nop);
                last.connect(first, Nop);
                self.translate_hir(repeat.inner(), pair(first, last))
            }
            //
            //                               ╭─────ε─────╮
            //                               ↓           │
            // (1)──'a'──...──'a'─→(n)──ε─→(n+1)──'a'─→(n+2)──ε─→(n+3)
            //
            (n, None) => {
                let mut first = sub.first;
                for _ in 1..n {
                    let last = self.graph.node();
                    self.translate_hir(repeat.inner(), pair(first, last));
                    first = last;
                }
                sub.first = first;
                let first = self.graph.node();
                let last = self.graph.node();
                self.translate_hir(repeat.inner(), pair(first, last));
                sub.first.connect(first, Nop);
                last.connect(sub.last, Nop);
                last.connect(first, Nop);
            }
            //
            // (0)──'a'──(1)──'a'──...──'a'─→(n)
            //
            (n, Some(m)) if n == m => {
                if n == 0 {
                    sub.first.connect(sub.last, Nop);
                } else {
                    let mut first = sub.first;
                    for _ in 0..n - 1 {
                        let last = self.graph.node();
                        self.translate_hir(repeat.inner(), pair(first, last));
                        first = last;
                    }
                    self.translate_hir(repeat.inner(), pair(first, sub.last));
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
                let mut first = sub.first;
                for _ in 0..n {
                    let last = self.graph.node();
                    self.translate_hir(repeat.inner(), pair(first, last));
                    first = last;
                }
                for _ in n..m {
                    let mid_one = self.graph.node();
                    first.connect(mid_one, Nop);
                    let mid_two = self.graph.node();
                    self.translate_hir(repeat.inner(), pair(mid_one, mid_two));
                    let last = self.graph.node();
                    mid_two.connect(last, Nop);
                    first.connect(sub.last, Nop);
                    first = last;
                }
                first.connect(sub.last, Nop);
            }
            (n, Some(m)) => {
                panic!("invalid repetition counters: {{{n},{m}}}");
            }
        }
    }

    fn translate_concat(&mut self, concat: &ConcatHir, sub: Pair<'a>) {
        let items = concat.items();
        if items.is_empty() {
            sub.first.connect(sub.last, Nop);
            return;
        }
        let mut first = sub.first;
        for hir in &items[..items.len() - 1] {
            let last = self.graph.node();
            self.translate_hir(hir, pair(first, last));
            first = last;
        }
        let hir = items.last().unwrap();
        self.translate_hir(hir, pair(first, sub.last));
    }

    /// ```txt
    ///  ╭───ε──→(○)──'a'─→(○)──ε───╮
    ///  │                          ↓
    /// (○)──ε──→(○)──'b'─→(○)──ε─→(○)
    ///  │                          ↑
    ///  ╰───ε──→(○)──'c'─→(○)──ε───╯
    /// ```
    fn translate_disjunct(&mut self, disjunct: &DisjunctHir, sub: Pair<'a>) {
        let mut tr_outs = Vec::new();
        for hir in disjunct.alternatives() {
            let first = self.graph.node();
            let last = self.graph.node();
            sub.first.connect(first, Nop);
            let tr_out = last.connect(sub.last, Nop);
            tr_outs.push(tr_out);
            self.translate_hir(hir, pair(first, last));
        }
    }

    pub fn next_reg(&self) -> u32 {
        let new_reg = self.next_reg.get();
        self.next_reg
            .update(|id| id.checked_add(1).expect("register id overflow"));
        new_reg
    }
}

#[cfg(test)]
#[path = "utest/translator.rs"]
mod utest;
