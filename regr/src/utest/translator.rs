use super::{Translator, pair};
use crate::graph::Graph;
use pretty_assertions::assert_eq;
use redt::{Range, SetU8, lit};
use resy::Hir;

#[test]
fn translate_literal() {
    fn tr(literal: &[u8]) -> String {
        let graph = Graph::new();
        let translator = Translator::new(&graph);
        let pair = pair(graph.node(), graph.node());
        translator.translate_literal(literal, pair);
        graph.to_string()
    }

    assert_eq!(
        tr(b""),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(1) {}
        )
    );
    assert_eq!(
        tr(b"ab"),
        lit!(
            ///node(0) {
            ///    ['a'] -> node(2)
            ///}
            ///node(2) {
            ///    ['b'] -> node(1)
            ///}
            ///node(1) {}
        )
    );
}

#[test]
fn translate_class() {
    fn tr(set: &SetU8) -> String {
        let graph = Graph::new();
        let translator = Translator::new(&graph);
        let pair = pair(graph.node(), graph.node());
        translator.translate_class(set, pair);
        graph.to_string()
    }

    let set = SetU8::new();
    set.insert_bytes(Range::new(4, 50));
    set.insert(100);
    assert_eq!(
        tr(&set),
        lit!(
            ///node(0) {
            ///    [04h-'2' | 'd'] -> node(1)
            ///}
            ///node(1) {}
        )
    );
}

#[test]
fn translate_repeat() {
    fn tr(repeat: &Hir) -> String {
        assert!(repeat.is_repeat());
        let graph = Graph::new();
        let mut translator = Translator::new(&graph);
        let pair = pair(graph.node(), graph.node());
        let Hir::Repeat(repeat) = repeat else {
            unreachable!()
        };
        translator.translate_repeat(repeat, pair);
        graph.to_string()
    }

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 0, None);
    assert_eq!(
        tr(&hir),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(2)
            ///    [Epsilon] -> node(1)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    [Epsilon] -> node(1)
            ///    [Epsilon] -> node(2)
            ///}
            ///node(1) {}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 1, None);
    assert_eq!(
        tr(&hir),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(2)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    [Epsilon] -> node(1)
            ///    [Epsilon] -> node(2)
            ///}
            ///node(1) {}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 3, None);
    assert_eq!(
        tr(&hir),
        lit!(
            ///node(0) {
            ///    ['a'] -> node(2)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    [Epsilon] -> node(4)
            ///}
            ///node(4) {
            ///    ['a'] -> node(5)
            ///}
            ///node(5) {
            ///    [Epsilon] -> node(1)
            ///    [Epsilon] -> node(4)
            ///}
            ///node(1) {}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 0, Some(0));
    assert_eq!(
        tr(&hir),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(1) {}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 3, Some(3));
    assert_eq!(
        tr(&hir),
        lit!(
            ///node(0) {
            ///    ['a'] -> node(2)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    ['a'] -> node(1)
            ///}
            ///node(1) {}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 1, Some(3));
    assert_eq!(
        tr(&hir),
        lit!(
            ///node(0) {
            ///    ['a'] -> node(2)
            ///}
            ///node(2) {
            ///    [Epsilon] -> node(3)
            ///    [Epsilon] -> node(1)
            ///}
            ///node(3) {
            ///    ['a'] -> node(4)
            ///}
            ///node(4) {
            ///    [Epsilon] -> node(5)
            ///}
            ///node(5) {
            ///    [Epsilon] -> node(6)
            ///    [Epsilon] -> node(1)
            ///}
            ///node(6) {
            ///    ['a'] -> node(7)
            ///}
            ///node(7) {
            ///    [Epsilon] -> node(8)
            ///}
            ///node(8) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(1) {}
        )
    );
}

#[test]
#[should_panic(expected = "invalid repetition counters: {3,2}")]
fn translate_repeat_fails() {
    let literal = Hir::literal("a");
    let repeat = Hir::repeat(literal, 3, Some(2));
    let graph = Graph::new();
    let mut translator = Translator::new(&graph);
    let sub = pair(graph.node(), graph.node());
    let Hir::Repeat(repeat) = repeat else {
        unreachable!()
    };
    translator.translate_repeat(&repeat, sub);
}

#[test]
fn translate_concat() {
    let concat = Hir::concat([]);
    let graph = Graph::new();
    let translator = Translator::new(&graph);
    let sub = pair(graph.node(), graph.node());
    let Hir::Literal(concat) = concat else {
        unreachable!()
    };
    translator.translate_literal(&concat, sub);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(1) {}
        )
    );

    let concat = Hir::concat([Hir::literal("a"), Hir::literal("b"), Hir::literal("c")]);
    let graph = Graph::new();
    let mut translator = Translator::new(&graph);
    let sub = pair(graph.node(), graph.node());
    let Hir::Concat(concat) = concat else {
        unreachable!()
    };
    translator.translate_concat(&concat, sub);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///node(0) {
            ///    ['a'] -> node(2)
            ///}
            ///node(2) {
            ///    ['b'] -> node(3)
            ///}
            ///node(3) {
            ///    ['c'] -> node(1)
            ///}
            ///node(1) {}
        )
    );
}

#[test]
fn translate_disjunct() {
    let disjunct = Hir::disjunct([Hir::literal("a"), Hir::literal("b"), Hir::literal("c")]);
    let graph = Graph::new();
    let mut translator = Translator::new(&graph);
    let sub = pair(graph.node(), graph.node());
    let Hir::Disjunct(disjunct) = disjunct else {
        unreachable!()
    };
    translator.translate_disjunct(&disjunct, sub);
    assert_eq!(
        graph.to_string(),
        lit!(
            ///node(0) {
            ///    [Epsilon] -> node(2)
            ///    [Epsilon] -> node(4)
            ///    [Epsilon] -> node(6)
            ///}
            ///node(2) {
            ///    ['a'] -> node(3)
            ///}
            ///node(3) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(1) {}
            ///node(4) {
            ///    ['b'] -> node(5)
            ///}
            ///node(5) {
            ///    [Epsilon] -> node(1)
            ///}
            ///node(6) {
            ///    ['c'] -> node(7)
            ///}
            ///node(7) {
            ///    [Epsilon] -> node(1)
            ///}
        )
    );
}
