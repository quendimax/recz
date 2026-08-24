use pretty_assertions::assert_eq;
use recz_adt::lit;
use recz_graph::Graph;
use recz_syntax::{Parser, Translator, codec::Utf8Codec};

fn parse(pattern: &str) -> String {
    let graph = Graph::new();
    let parser = Parser::new(Utf8Codec);
    let hir = parser.parse(pattern).unwrap();
    let mut translator = Translator::new(&graph);
    let start_node = graph.start_node();
    let end_node = graph.node();
    translator.translate(&hir, start_node, end_node);
    graph.to_string()
}

#[test]
fn translate_literal() {
    assert_eq!(
        parse("sun"),
        lit!(
            ///graph {
            ///  no_0 { 's' -> no_2 }
            ///  no_1 {}
            ///  no_2 { 'u' -> no_3 }
            ///  no_3 { 'n' -> no_1 }
            ///}
        )
    );
}

#[test]
fn translate_class() {
    assert_eq!(
        parse("[a-ce]"),
        lit!(
            ///graph {
            ///  no_0 {
            ///    EPS -> no_2
            ///    EPS -> no_4
            ///  }
            ///  no_1 {}
            ///  no_2 { 'a'-'c' -> no_3 }
            ///  no_3 { EPS -> no_1 }
            ///  no_4 { 'e' -> no_5 }
            ///  no_5 { EPS -> no_1 }
            ///}
        )
    );
    assert_eq!(
        parse("[a-я]"),
        lit!(
            ///graph {
            ///  no_0 {
            ///    EPS -> no_2
            ///    EPS -> no_4
            ///    EPS -> no_7
            ///  }
            ///  no_1 {}
            ///  no_2 { 'a'-'\x7F' -> no_3 }
            ///  no_3 { EPS -> no_1 }
            ///  no_4 { '\xC2'-'\xD0' -> no_6 }
            ///  no_5 { EPS -> no_1 }
            ///  no_6 { '\x80'-'\xBF' -> no_5 }
            ///  no_7 { '\xD1' -> no_9 }
            ///  no_8 { EPS -> no_1 }
            ///  no_9 { '\x80'-'\x8F' -> no_8 }
            ///}
        )
    );
}

#[test]
fn translate_group_0_0() {
    assert_eq!(
        parse("(?D<1>)"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_2 }
            ///  no_1 {}
            ///  no_2 { EPS -> no_3 }
            ///  no_3 { EPS / -g0 -> no_1 }
            ///}
        )
    );
}

#[test]
fn translate_group_0_1() {
    assert_eq!(
        parse("(?D<0>a)b(?D<1>c)"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_3 }
            ///  no_1 {}
            ///  no_2 { 'b' -> no_5 }
            ///  no_3 { 'a' -> no_4 }
            ///  no_4 { EPS / -g0 -> no_2 }
            ///  no_5 { EPS / +g1 -> no_6 }
            ///  no_6 { 'c' -> no_7 }
            ///  no_7 { EPS / -g1 -> no_1 }
            ///}
        )
    );
}

#[test]
fn translate_group_0_2() {
    assert_eq!(
        parse("(?D<0>)(?D<1>)(?D<234>)"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_3 }
            ///  no_1 {}
            ///  no_2 { EPS / +g1 -> no_6 }
            ///  no_3 { EPS -> no_4 }
            ///  no_4 { EPS / -g0 -> no_2 }
            ///  no_5 { EPS / +g2 -> no_8 }
            ///  no_6 { EPS -> no_7 }
            ///  no_7 { EPS / -g1 -> no_5 }
            ///  no_8 { EPS -> no_9 }
            ///  no_9 { EPS / -g2 -> no_1 }
            ///}
        )
    );
}

#[test]
fn translate_group_1() {
    assert_eq!(
        parse("(?D<1>)(a|bc)(?D<2>)"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_3 }
            ///  no_1 {}
            ///  no_2 {
            ///    EPS -> no_6
            ///    EPS -> no_8
            ///  }
            ///  no_3 { EPS -> no_4 }
            ///  no_4 { EPS / -g0 -> no_2 }
            ///  no_5 { EPS / +g1 -> no_11 }
            ///  no_6 { 'a' -> no_7 }
            ///  no_7 { EPS -> no_5 }
            ///  no_8 { 'b' -> no_10 }
            ///  no_9 { EPS -> no_5 }
            ///  no_10 { 'c' -> no_9 }
            ///  no_11 { EPS -> no_12 }
            ///  no_12 { EPS / -g1 -> no_1 }
            ///}
        )
    );
}

#[test]
fn translate_group_2() {
    assert_eq!(
        parse("(?D<1>)(a|b)(?D<2>)"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_3 }
            ///  no_1 {}
            ///  no_2 {
            ///    EPS -> no_6
            ///    EPS -> no_8
            ///  }
            ///  no_3 { EPS -> no_4 }
            ///  no_4 { EPS / -g0 -> no_2 }
            ///  no_5 { EPS / +g1 -> no_10 }
            ///  no_6 { 'a' -> no_7 }
            ///  no_7 { EPS -> no_5 }
            ///  no_8 { 'b' -> no_9 }
            ///  no_9 { EPS -> no_5 }
            ///  no_10 { EPS -> no_11 }
            ///  no_11 { EPS / -g1 -> no_1 }
            ///}
        )
    );
}

#[test]
fn translate_group_3() {
    assert_eq!(
        parse("(?D<1>)((?D<2>a)|b(?D<3>a))"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_3 }
            ///  no_1 {}
            ///  no_2 {
            ///    EPS -> no_5
            ///    EPS -> no_9
            ///  }
            ///  no_3 { EPS -> no_4 }
            ///  no_4 { EPS / -g0 -> no_2 }
            ///  no_5 { EPS / +g1 -> no_7 }
            ///  no_6 { EPS / !g2 -> no_1 }
            ///  no_7 { 'a' -> no_8 }
            ///  no_8 { EPS / -g1 -> no_6 }
            ///  no_9 { 'b' -> no_11 }
            ///  no_10 { EPS / !g1 -> no_1 }
            ///  no_11 { EPS / +g2 -> no_12 }
            ///  no_12 { 'a' -> no_13 }
            ///  no_13 { EPS / -g2 -> no_10 }
            ///}
        )
    );
}

#[test]
fn translate_group_4() {
    assert_eq!(
        parse("(?D<1>((?D<2>a((?D<3>d)|(?D<5>e)))|b(?D<4>a)))"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_2 }
            ///  no_1 {}
            ///  no_2 {
            ///    EPS -> no_4
            ///    EPS -> no_17
            ///  }
            ///  no_3 { EPS / -g0 -> no_1 }
            ///  no_4 { EPS / +g1 -> no_6 }
            ///  no_5 { EPS / !g4 -> no_3 }
            ///  no_6 { 'a' -> no_8 }
            ///  no_7 { EPS / -g1 -> no_5 }
            ///  no_8 {
            ///    EPS -> no_9
            ///    EPS -> no_13
            ///  }
            ///  no_9 { EPS / +g2 -> no_11 }
            ///  no_10 { EPS / !g3 -> no_7 }
            ///  no_11 { 'd' -> no_12 }
            ///  no_12 { EPS / -g2 -> no_10 }
            ///  no_13 { EPS / +g3 -> no_15 }
            ///  no_14 { EPS / !g2 -> no_7 }
            ///  no_15 { 'e' -> no_16 }
            ///  no_16 { EPS / -g3 -> no_14 }
            ///  no_17 { 'b' -> no_19 }
            ///  no_18 { EPS / !g1,!g2,!g3 -> no_3 }
            ///  no_19 { EPS / +g4 -> no_20 }
            ///  no_20 { 'a' -> no_21 }
            ///  no_21 { EPS / -g4 -> no_18 }
            ///}
        )
    );
}

#[test]
fn translate_group_5() {
    assert_eq!(
        parse("(?D<1>a)(?D<2>b)*(?D<3>c)"),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_3 }
            ///  no_1 {}
            ///  no_2 {
            ///    EPS -> no_6
            ///    EPS -> no_5
            ///  }
            ///  no_3 { 'a' -> no_4 }
            ///  no_4 { EPS / -g0 -> no_2 }
            ///  no_5 { EPS / +g2 -> no_10 }
            ///  no_6 { EPS / +g1 -> no_8 }
            ///  no_7 {
            ///    EPS -> no_5
            ///    EPS -> no_6
            ///  }
            ///  no_8 { 'b' -> no_9 }
            ///  no_9 { EPS / -g1 -> no_7 }
            ///  no_10 { 'c' -> no_11 }
            ///  no_11 { EPS / -g2 -> no_1 }
            ///}
        )
    );
}
