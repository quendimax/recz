use gungraun::prelude::*;
use gungraun::{Callgrind, FlamegraphConfig};
use recz_adt::{RangeU8, SetU8};
use recz_graph::{Graph, algo};
use recz_syntax::{Hir, Parser, Translator, codec::Utf8Codec};
use std::hint::black_box;

#[library_benchmark]
#[benches::one(args = [0..=0, 0..=65, 3..=160, 1..=250])]
fn rangeu8_into_setu8(range: impl Into<RangeU8>) -> SetU8 {
    black_box(SetU8::from(black_box(&range.into())))
}

library_benchmark_group!(name = adt, benchmarks = rangeu8_into_setu8);

#[library_benchmark]
#[benches::one(args = ["hello", "aa*", ".*(?D<0>aa*)"])]
fn parse_regex(regex: &str) -> Hir {
    let parser = Parser::new(Utf8Codec);
    parser.parse(regex).unwrap()
}

library_benchmark_group!(name = syntax, benchmarks = parse_regex);

#[library_benchmark]
#[benches::one(args = ["hello", "aa*", ".*(?D<0>aa*)"])]
fn build_dfa(regex: &str) -> Graph {
    let parser = Parser::new(Utf8Codec);
    let hir = parser.parse(regex).unwrap();
    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());
    let dfa = algo::determine(&nfa);
    black_box(dfa)
}

library_benchmark_group!(name = full, benchmarks = build_dfa);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default().flamegraph(FlamegraphConfig::default())),
    library_benchmark_groups = [adt, syntax, full]
);
