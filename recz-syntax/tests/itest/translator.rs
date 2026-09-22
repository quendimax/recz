use pretty_assertions::assert_eq;
use recz_adt::lit;
use recz_graph::Graph;
use recz_syntax::{Parser, Result, Translator, codec::Utf8Codec};

fn parse(pattern: &str) -> Result<String> {
    let graph = Graph::new();
    let parser = Parser::new(Utf8Codec);
    let hir = parser.parse(pattern)?;
    let mut translator = Translator::new(&graph);
    translator.translate(&hir, graph.start_node()).finalize();
    Ok(graph.to_string())
}

#[test]
fn translate_literal() {
    assert_eq!(
        parse("sun").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { 's' -> no_1 }
            ///  no_1 { 'u' -> no_2 }
            ///  no_2 { 'n' -> fi_3 }
            ///  fi_3 {}
            ///}
        )
    );
}

#[test]
fn translate_class() {
    assert_eq!(
        parse("[a-ce]").unwrap(),
        lit!(
            ///graph {
            ///  no_0 {
            ///    EPS -> no_1
            ///    EPS -> no_3
            ///  }
            ///  no_1 { 'a'-'c' -> no_2 }
            ///  no_2 { EPS -> fi_5 }
            ///  no_3 { 'e' -> no_4 }
            ///  no_4 { EPS -> fi_5 }
            ///  fi_5 {}
            ///}
        )
    );
    assert_eq!(
        parse("[a-я]").unwrap(),
        lit!(
            ///graph {
            ///  no_0 {
            ///    EPS -> no_1
            ///    EPS -> no_3
            ///    EPS -> no_6
            ///  }
            ///  no_1 { 'a'-'\x7F' -> no_2 }
            ///  no_2 { EPS -> fi_9 }
            ///  no_3 { '\xC2'-'\xD0' -> no_4 }
            ///  no_4 { '\x80'-'\xBF' -> no_5 }
            ///  no_5 { EPS -> fi_9 }
            ///  no_6 { '\xD1' -> no_7 }
            ///  no_7 { '\x80'-'\x8F' -> no_8 }
            ///  no_8 { EPS -> fi_9 }
            ///  fi_9 {}
            ///}
        )
    );
}

#[test]
fn translate_group_0_1() {
    assert_eq!(
        parse("(?D<2>a)b(?D<1>c)").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 { EPS / -g0 -> no_3 }
            ///  no_3 { 'b' -> no_4 }
            ///  no_4 { EPS / +g1 -> no_5 }
            ///  no_5 { 'c' -> no_6 }
            ///  no_6 { EPS / -g1 -> fi_7 }
            ///  fi_7 {}
            ///}
        )
    );
}

#[test]
fn translate_empty_group() {
    assert_eq!(
        parse("(?D<3>)(?D<1>)(?D<234>)").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 { EPS -> no_2 }
            ///  no_2 { EPS / -g0 -> no_3 }
            ///  no_3 { EPS / +g1 -> no_4 }
            ///  no_4 { EPS -> no_5 }
            ///  no_5 { EPS / -g1 -> no_6 }
            ///  no_6 { EPS / +g2 -> no_7 }
            ///  no_7 { EPS -> no_8 }
            ///  no_8 { EPS / -g2 -> fi_9 }
            ///  fi_9 {}
            ///}
        )
    );
}

#[test]
fn translate_group_1() {
    assert_eq!(
        parse("(?D<1>)(a|bc)(?D<2>)").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 { EPS -> no_2 }
            ///  no_2 { EPS / -g0 -> no_3 }
            ///  no_3 {
            ///    EPS -> no_4
            ///    EPS -> no_6
            ///  }
            ///  no_4 { 'a' -> no_5 }
            ///  no_5 { EPS -> no_9 }
            ///  no_6 { 'b' -> no_7 }
            ///  no_7 { 'c' -> no_8 }
            ///  no_8 { EPS -> no_9 }
            ///  no_9 { EPS / +g1 -> no_10 }
            ///  no_10 { EPS -> no_11 }
            ///  no_11 { EPS / -g1 -> fi_12 }
            ///  fi_12 {}
            ///}
        )
    );
}

#[test]
fn translate_group_2() {
    assert_eq!(
        parse("(?D<1>)(a|b)(?D<2>)").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 { EPS -> no_2 }
            ///  no_2 { EPS / -g0 -> no_3 }
            ///  no_3 {
            ///    EPS -> no_4
            ///    EPS -> no_6
            ///  }
            ///  no_4 { 'a' -> no_5 }
            ///  no_5 { EPS -> no_8 }
            ///  no_6 { 'b' -> no_7 }
            ///  no_7 { EPS -> no_8 }
            ///  no_8 { EPS / +g1 -> no_9 }
            ///  no_9 { EPS -> no_10 }
            ///  no_10 { EPS / -g1 -> fi_11 }
            ///  fi_11 {}
            ///}
        )
    );
}

#[test]
fn translate_group_3() {
    assert_eq!(
        parse("(?D<1>)((?D<2>a)|b(?D<3>a))").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 { EPS -> no_2 }
            ///  no_2 { EPS / -g0 -> no_3 }
            ///  no_3 {
            ///    EPS -> no_4
            ///    EPS -> no_8
            ///  }
            ///  no_4 { EPS / +g1 -> no_5 }
            ///  no_5 { 'a' -> no_6 }
            ///  no_6 { EPS / -g1 -> no_7 }
            ///  no_7 { EPS / !g2 -> fi_13 }
            ///  no_8 { 'b' -> no_9 }
            ///  no_9 { EPS / +g2 -> no_10 }
            ///  no_10 { 'a' -> no_11 }
            ///  no_11 { EPS / -g2 -> no_12 }
            ///  no_12 { EPS / !g1 -> fi_13 }
            ///  fi_13 {}
            ///}
        )
    );
}

#[test]
fn translate_group_4() {
    assert_eq!(
        parse("(?D<1>((?D<2>a((?D<3>d)|(?D<5>e)))|b(?D<4>a)))").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 {
            ///    EPS -> no_2
            ///    EPS -> no_15
            ///  }
            ///  no_2 { EPS / +g1 -> no_3 }
            ///  no_3 { 'a' -> no_4 }
            ///  no_4 {
            ///    EPS -> no_5
            ///    EPS -> no_9
            ///  }
            ///  no_5 { EPS / +g2 -> no_6 }
            ///  no_6 { 'd' -> no_7 }
            ///  no_7 { EPS / -g2 -> no_8 }
            ///  no_8 { EPS / !g3 -> no_13 }
            ///  no_9 { EPS / +g3 -> no_10 }
            ///  no_10 { 'e' -> no_11 }
            ///  no_11 { EPS / -g3 -> no_12 }
            ///  no_12 { EPS / !g2 -> no_13 }
            ///  no_13 { EPS / -g1 -> no_14 }
            ///  no_14 { EPS / !g4 -> no_20 }
            ///  no_15 { 'b' -> no_16 }
            ///  no_16 { EPS / +g4 -> no_17 }
            ///  no_17 { 'a' -> no_18 }
            ///  no_18 { EPS / -g4 -> no_19 }
            ///  no_19 { EPS / !g1 -> no_20 }
            ///  no_20 { EPS / -g0 -> fi_21 }
            ///  fi_21 {}
            ///}
        )
    );
}

#[test]
fn translate_group_5() {
    assert_eq!(
        parse("(?D<1>a)(?D<2>b)*(?D<3>c)").unwrap(),
        lit!(
            ///graph {
            ///  no_0 { EPS / +g0 -> no_1 }
            ///  no_1 { 'a' -> no_2 }
            ///  no_2 { EPS / -g0 -> no_3 }
            ///  no_3 {
            ///    EPS -> no_4
            ///    EPS -> no_8
            ///  }
            ///  no_4 { EPS / +g1 -> no_5 }
            ///  no_5 { 'b' -> no_6 }
            ///  no_6 { EPS / -g1 -> no_7 }
            ///  no_7 {
            ///    EPS -> no_4
            ///    EPS -> no_8
            ///  }
            ///  no_8 { EPS / +g2 -> no_9 }
            ///  no_9 { 'c' -> no_10 }
            ///  no_10 { EPS / -g2 -> fi_11 }
            ///  fi_11 {}
            ///}
        )
    );
}
