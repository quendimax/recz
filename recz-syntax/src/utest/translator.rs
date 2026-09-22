use super::Translator;
use crate::hir::Hir;
use pretty_assertions::assert_eq;
use recz_adt::{Range, SetU8, lit};
use recz_graph::Graph;

#[test]
fn translate_literal() {
    fn tr(literal: &[u8]) -> String {
        let graph = Graph::new();
        let translator = Translator::new(&graph);
        translator.translate_literal(literal, graph.node());
        graph.to_string()
    }

    assert_eq!(
        tr(b""),
        lit!(
            ///graph {
            ///  no_0 { EPS -> no_1 }
            ///  no_1 {}
            ///}
        )
    );
    assert_eq!(
        tr(b"ab"),
        lit!(
            ///graph {
            ///  no_0 { 'a' -> no_1 }
            ///  no_1 { 'b' -> no_2 }
            ///  no_2 {}
            ///}
        )
    );
}

#[test]
fn translate_class() {
    fn tr(set: &SetU8) -> String {
        let graph = Graph::new();
        let translator = Translator::new(&graph);
        translator.translate_class(set, graph.node());
        graph.to_string()
    }

    let set = SetU8::new();
    set.insert_bytes(Range::new(4, 50));
    set.insert(100);
    assert_eq!(
        tr(&set),
        lit!(
            ///graph {
            ///  no_0 { '\x04'-'2' | 'd' -> no_1 }
            ///  no_1 {}
            ///}
        )
    );
}

#[test]
fn translate_repeat() {
    fn tr(repeat: &Hir) -> String {
        assert!(repeat.is_repeat());
        let graph = Graph::new();
        let mut translator = Translator::new(&graph);
        let Hir::Repeat(repeat) = repeat else {
            unreachable!()
        };
        translator.translate_repeat(repeat, graph.node());
        graph.to_string()
    }

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 0, None);
    assert_eq!(
        tr(&hir),
        lit!(
            ///graph {
            ///  no_0 {
            ///    EPS -> no_1
            ///    EPS -> no_3
            ///  }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 {
            ///    EPS -> no_1
            ///    EPS -> no_3
            ///  }
            ///  no_3 {}
            ///}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 1, None);
    assert_eq!(
        tr(&hir),
        lit!(
            ///graph {
            ///  no_0 { EPS -> no_1 }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 {
            ///    EPS -> no_1
            ///    EPS -> no_3
            ///  }
            ///  no_3 {}
            ///}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 3, None);
    assert_eq!(
        tr(&hir),
        lit!(
            ///graph {
            ///  no_0 { 'a' -> no_1 }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 { EPS -> no_3 }
            ///  no_3 { 'a' -> no_4 }
            ///  no_4 {
            ///    EPS -> no_3
            ///    EPS -> no_5
            ///  }
            ///  no_5 {}
            ///}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 0, Some(0));
    assert_eq!(
        tr(&hir),
        lit!(
            ///graph {
            ///  no_0 { EPS -> no_1 }
            ///  no_1 {}
            ///}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 3, Some(3));
    assert_eq!(
        tr(&hir),
        lit!(
            ///graph {
            ///  no_0 { 'a' -> no_1 }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 { 'a' -> no_3 }
            ///  no_3 {}
            ///}
        )
    );

    let literal = Hir::literal("a");
    let hir = Hir::repeat(literal, 1, Some(3));
    assert_eq!(
        tr(&hir),
        lit!(
            ///graph {
            ///  no_0 { 'a' -> no_1 }
            ///  no_1 {
            ///    EPS -> no_2
            ///    EPS -> no_7
            ///  }
            ///  no_2 { 'a' -> no_3 }
            ///  no_3 { EPS -> no_4 }
            ///  no_4 {
            ///    EPS -> no_5
            ///    EPS -> no_7
            ///  }
            ///  no_5 { 'a' -> no_6 }
            ///  no_6 { EPS -> no_7 }
            ///  no_7 {}
            ///}
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
    let Hir::Repeat(repeat) = repeat else {
        unreachable!()
    };
    translator.translate_repeat(&repeat, graph.node());
}

#[test]
fn translate_concat() {
    let concat = Hir::concat([]);
    let graph = Graph::new();
    let translator = Translator::new(&graph);
    let Hir::Literal(concat) = concat else {
        unreachable!()
    };
    translator.translate_literal(&concat, graph.node());
    assert_eq!(
        graph.to_string(),
        lit!(
            ///graph {
            ///  no_0 { EPS -> no_1 }
            ///  no_1 {}
            ///}
        )
    );

    let concat = Hir::concat([Hir::literal("a"), Hir::literal("b"), Hir::literal("c")]);
    let graph = Graph::new();
    let mut translator = Translator::new(&graph);
    let Hir::Concat(concat) = concat else {
        unreachable!()
    };
    translator.translate_concat(&concat, graph.node());
    assert_eq!(
        graph.to_string(),
        lit!(
            ///graph {
            ///  no_0 { 'a' -> no_1 }
            ///  no_1 { 'b' -> no_2 }
            ///  no_2 { 'c' -> no_3 }
            ///  no_3 {}
            ///}
        )
    );
}

#[test]
fn translate_disjunct() {
    let disjunct = Hir::disjunct([Hir::literal("a"), Hir::literal("b"), Hir::literal("c")]);
    let graph = Graph::new();
    let mut translator = Translator::new(&graph);
    let Hir::Disjunct(disjunct) = disjunct else {
        unreachable!()
    };
    translator.translate_disjunct(&disjunct, graph.node());
    assert_eq!(
        graph.to_string(),
        lit!(
            ///graph {
            ///  no_0 {
            ///    EPS -> no_1
            ///    EPS -> no_3
            ///    EPS -> no_5
            ///  }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 { EPS -> no_7 }
            ///  no_3 { 'b' -> no_4 }
            ///  no_4 { EPS -> no_7 }
            ///  no_5 { 'c' -> no_6 }
            ///  no_6 { EPS -> no_7 }
            ///  no_7 {}
            ///}
        )
    );
}
