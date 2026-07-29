use pretty_assertions::assert_eq;
use recz_adt::{RangeU8, range};
use recz_graph::{Edge, Graph, Tag};
use smallvec::SmallVec;

type Chunk = u64;

fn single(sym: u8) -> RangeU8 {
    range(sym, sym)
}

fn handle_edge<F, R>(f: F) -> R
where
    F: Fn(Edge<'_>) -> R,
{
    let gr = Graph::new();
    let edge = gr.node().connect(gr.node());
    f(edge)
}

fn handle_edge_from_chunks<F, R>(chunks: &[u64; 4], f: F) -> R
where
    F: Fn(Edge<'_>) -> R,
{
    let gr = Graph::new();
    let edge = gr.node().connect(gr.node());
    let mut sym = 0u8;
    for chunk in chunks {
        let mut mask = 1u64;
        for _ in 0..64 {
            if mask & *chunk != 0 {
                edge.add_symbol(sym);
            }
            mask <<= 1;
            sym = sym.wrapping_add(1);
        }
    }
    f(edge)
}

fn handle_edge_from_symbols<F, R>(symbols: &[u8], f: F) -> R
where
    F: Fn(Edge<'_>) -> R,
{
    let gr = Graph::new();
    let edge = gr.node().connect(gr.node());
    for sym in symbols {
        edge.add_symbol(*sym);
    }
    f(edge)
}

fn handle_epsilon<F, R>(f: F) -> R
where
    F: Fn(Edge<'_>) -> R,
{
    let gr = Graph::new();
    let edge = gr.node().connect(gr.node());
    f(edge)
}

#[test]
fn edge_clone() {
    handle_edge(|edge| assert_eq!(edge, edge.clone()));
}

#[test]
fn edge_symbols() {
    type Vec = SmallVec<[u8; 8]>;
    fn symbols(a: u64, b: u64, c: u64, d: u64) -> Vec {
        handle_edge_from_chunks(&[a, b, c, d], |tr| tr.symbols().collect::<Vec>())
    }
    fn vec<const N: usize>(buf: [u8; N]) -> Vec {
        Vec::from(&buf as &[u8])
    }

    assert_eq!(symbols(0, 0, 0, 0), vec([]));
    assert_eq!(symbols(255, 0, 0, 0), (0..=7).collect::<Vec>());
    assert_eq!(symbols(u64::MAX, 0, 0, 0), (0..=63).collect::<Vec>());
    assert_eq!(symbols(u64::MAX, 255, 0, 0), (0..=71).collect::<Vec>());
    assert_eq!(symbols(0x8000000000000001, 1, 0, 0), vec([0, 63, 64]));
    assert_eq!(symbols(0x5555, 0, 0, 0), vec([0, 2, 4, 6, 8, 10, 12, 14]));
    assert_eq!(
        symbols(u64::MAX, u64::MAX, u64::MAX, u64::MAX),
        (0..=255).collect::<Vec>()
    );

    handle_epsilon(|edge| assert_eq!(edge.symbols().next(), None));
}

#[test]
fn edge_ranges() {
    type Vec = SmallVec<[RangeU8; 4]>;
    fn ranges(a: u64, b: u64, c: u64, d: u64) -> Vec {
        handle_edge_from_chunks(&[a, b, c, d], |tr| tr.ranges().collect::<Vec>())
    }
    fn vec<const N: usize>(buf: [RangeU8; N]) -> Vec {
        Vec::from(&buf as &[RangeU8])
    }

    assert_eq!(ranges(0, 0, 0, 0), vec([]));
    assert_eq!(ranges(255, 0, 0, 0), vec([range(0, 7)]));
    assert_eq!(ranges(255, 255, 0, 0), vec([range(0, 7), range(64, 71)]));
    assert_eq!(ranges(0, 255, 0, 0), vec([range(64, 71)]));
    assert_eq!(ranges(0, 0, 0, 255), vec([range(192, 199)]));
    assert_eq!(
        ranges(255, 255, 255, 255),
        vec([range(0, 7), range(64, 71), range(128, 135), range(192, 199)])
    );
    assert_eq!(ranges(u64::MAX, 0, 0, 0), vec([range(0, 63)]));
    assert_eq!(ranges(0, u64::MAX, 0, 0), vec([range(64, 127)]));
    assert_eq!(ranges(0, 0, u64::MAX, 0), vec([range(128, 191)]));
    assert_eq!(ranges(0, 0, 0, u64::MAX), vec([range(192, 255)]));
    assert_eq!(
        ranges(u64::MAX, 0, 0, u64::MAX),
        vec([range(0, 63), range(192, 255)])
    );
    assert_eq!(
        ranges(u64::MAX, u64::MAX, u64::MAX, u64::MAX),
        vec([range(0, 255)])
    );
    assert_eq!(ranges(1, 0, 0, 0), vec([single(0)]));
    assert_eq!(
        ranges(0x8000000000000001, 0, 0, 0),
        vec([single(0), single(63)])
    );
    assert_eq!(
        ranges(0x8000000000000001, 0x8000000000000001, 0, 0),
        vec([single(0), range(63, 64), single(127)])
    );
    assert_eq!(
        ranges(0xC000000000000007, 0x1F000001, 0, 0),
        vec([range(0, 2), range(62, 64), range(88, 92)])
    );

    handle_epsilon(|edge| assert_eq!(edge.ranges().next(), None));
}

#[test]
fn edge_contains_symbol() {
    handle_edge_from_symbols(b"\x00bcde\xFF", |edge| {
        assert_eq!(edge.contains_symbol(0), true);
        assert_eq!(edge.contains_symbol(255), true);
        assert_eq!(edge.contains_symbol(b'b'), true);
        assert_eq!(edge.contains_symbol(b'c'), true);
        assert_eq!(edge.contains_symbol(b'f'), false);
        assert_eq!(edge.contains_symbol(254), false);
    });
}

#[test]
fn edge_contains_range() {
    handle_edge_from_symbols(&[0, 1, 5, 6, 7, 255], |edge| {
        assert!(edge.contains_symbols(single(0)));
        assert!(edge.contains_symbols(range(0, 1)));
        assert!(edge.contains_symbols(range(5, 7)));
        assert!(edge.contains_symbols(single(255)));
        assert!(!edge.contains_symbols(range(0, 3)));
        assert!(!edge.contains_symbols(range(2, 4)));
        assert!(!edge.contains_symbols(single(254)));
    });

    handle_edge_from_chunks(&[Chunk::MAX, Chunk::MAX, Chunk::MAX, Chunk::MAX], |edge| {
        assert!(edge.contains_symbols(range(0, 100)));
        assert!(edge.contains_symbols(range(0, 160)));
        assert!(edge.contains_symbols(range(0, 255)));
    });
}

#[test]
fn edge_contains_edge() {
    let gr = Graph::new();
    let edge_a = gr.node().connect(gr.node());
    edge_a.add_symbol(b'a');
    edge_a.add_symbol(b'c');
    edge_a.add_symbol(b'e');
    let edge_b = gr.node().connect(gr.node());
    edge_b.add_symbol(b'b');
    edge_b.add_symbol(b'd');
    edge_b.add_symbol(b'f');
    let edge_c = gr.node().connect(gr.node());
    edge_c.add_symbol(b'a');
    edge_c.add_symbol(b'b');
    edge_c.add_symbol(b'c');
    edge_c.add_symbol(b'd');
    edge_c.add_symbol(b'e');
    edge_c.add_symbol(b'f');
    edge_c.add_symbol(b'g');
    assert!(edge_a.is_superset(edge_a));
    assert!(edge_b.is_superset(edge_b));
    assert!(edge_c.is_superset(edge_c));
    assert!(edge_c.is_superset(edge_a));
    assert!(edge_c.is_superset(edge_a));
    assert!(!edge_a.is_superset(edge_b));
    assert!(!edge_a.is_superset(edge_c));
    assert!(!edge_b.is_superset(edge_a));
    assert!(!edge_b.is_superset(edge_c));
}

#[test]
fn edge_intersects_edge() {
    let gr = Graph::new();
    let edge_a = gr.node().connect(gr.node());
    edge_a.add_symbol(b'a');
    edge_a.add_symbol(b'c');
    edge_a.add_symbol(b'e');
    let edge_b = gr.node().connect(gr.node());
    edge_b.add_symbol(b'b');
    edge_b.add_symbol(b'd');
    edge_b.add_symbol(b'f');
    let edge_c = gr.node().connect(gr.node());
    edge_c.add_symbol(b'a');
    edge_c.add_symbol(b'b');
    edge_c.add_symbol(b'c');
    edge_c.add_symbol(b'd');
    edge_c.add_symbol(b'e');
    edge_c.add_symbol(b'f');
    assert_eq!(edge_a.intersects(edge_b), false);
    assert_eq!(edge_a.intersects(edge_c), true);
    assert_eq!(edge_b.intersects(edge_c), true);
}

#[test]
fn edge_merge_range() {
    fn check(range: impl Into<RangeU8>) -> Option<RangeU8> {
        let range = range.into();
        let gr = Graph::new();
        let edge = gr.node().connect(gr.node());
        edge.add_symbols(range);
        let mut range: Option<RangeU8> = None;
        for next_range in edge.ranges() {
            range = if let Some(range) = range {
                Some(range.merge(&next_range))
            } else {
                Some(next_range)
            }
        }
        range
    }
    assert_eq!(check(0..=2), Some(range(0, 2)));
    assert_eq!(check(3..=12), Some(range(3, 12)));
    assert_eq!(check(0..=63), Some(range(0, 63)));
    assert_eq!(check(0..=100), Some(range(0, 100)));
    assert_eq!(check(63..=127), Some(range(63, 127)));
    assert_eq!(check(63..=200), Some(range(63, 200)));
    assert_eq!(check(0..=255), Some(range(0, 255)));
    assert_eq!(check(192..=255), Some(range(192, 255)));
}

#[test]
fn edge_instruct() {
    let gr = Graph::new();
    let tag = Tag::OpenGroup(1);
    let edge_a = gr.node().connect(gr.node());
    edge_a.add_tag(tag);
    edge_a.add_symbol(b'a');
    edge_a.add_symbol(b'b');
    edge_a.add_symbol(b'c');
    edge_a.add_symbol(b'e');

    assert_eq!(format!("{edge_a}"), "['a'-'c' | 'e'] / +g1");
}

#[test]
fn edge_display_fmt() {
    fn tr(bytes: &[u8]) -> String {
        handle_edge_from_symbols(bytes, |tr| format!("{tr}"))
    }
    assert_eq!(tr(b""), "[Epsilon]");
    assert_eq!(tr(b"abc"), "['a'-'c']");
    assert_eq!(tr(b"abc"), "['a'-'c']");
    assert_eq!(tr(b"abcE"), "['E' | 'a'-'c']");
    assert_eq!(tr(b"?@"), "['?'-'@']");

    handle_edge(|edge| {
        edge.add_symbols(range(2, 4));
        edge.add_symbols(range(5, 6));
        assert_eq!(format!("{edge}"), "[02h-06h]");
    });
}

#[test]
fn edge_display_fmt_with_epsilon() {
    handle_epsilon(|edge| assert_eq!(format!("{}", edge), "[Epsilon]"));
    handle_edge_from_symbols(b"abc", |edge| {
        assert_eq!(format!("{edge}"), "['a'-'c']");
        edge.add_symbol(u8::MAX);
        assert_eq!(format!("{edge}"), "['a'-'c' | FFh]");
    });
}

#[test]
fn edge_debug_fmt() {
    fn tr(bytes: &[u8]) -> String {
        handle_edge_from_symbols(bytes, |edge| format!("{edge:?}"))
    }
    assert_eq!(tr(b""), "[Epsilon]");
    assert_eq!(tr(b"abc"), "[97-99]");
    assert_eq!(tr(b"abc"), "[97-99]");
    assert_eq!(tr(b"abcE"), "[69 | 97-99]");
    assert_eq!(tr(b"?@"), "[63-64]");

    handle_edge(|edge| {
        edge.add_symbols(range(2, 4));
        edge.add_symbols(range(5, 6));
        assert_eq!(format!("{edge}"), "[02h-06h]");
    });
}

#[test]
fn edge_debug_fmt_with_epsilon() {
    handle_epsilon(|edge| assert_eq!(format!("{edge:?}"), "[Epsilon]"));
    handle_edge_from_symbols(b"?@ABC", |edge| {
        assert_eq!(format!("{edge:?}"), "[63-67]");
        edge.add_symbol(u8::MAX);
        assert_eq!(format!("{edge:?}"), "[63-67 | 255]");
    });
}
