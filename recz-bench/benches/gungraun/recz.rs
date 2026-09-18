//! Benchmarks for every step of turning a pattern string into generated
//! matcher code: parsing (HIR), translating (NFA), determinization (DFA),
//! and code generation. Each step's `setup` function does all of the
//! preceding steps unmeasured, so every benchmark here covers exactly one
//! stage of the pipeline.

use gungraun::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use recz_codegen::{CodeGen, Config};
use recz_graph::{Graph, algo};
use recz_syntax::{Hir, Parser, Translator, codec::Utf8Codec};
use std::hint::black_box;

fn parse(regex: &str) -> Hir {
    let parser = Parser::new(Utf8Codec);
    parser.parse(regex).unwrap()
}

// The implicit whole-match capture group (`(?D<0>..)`) that `recz-macro`/
// `recz-cli` wrap every pattern's HIR in before translating: required
// structurally by later stages (code generation indexes capture group 0
// unconditionally), but not part of parsing itself.
fn parse_and_group(regex: &str) -> Hir {
    Hir::group(0u32, parse(regex))
}

fn translate(hir: &Hir) -> Graph {
    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(hir, nfa.start_node(), nfa.node().finalize());
    nfa
}

fn build_nfa(regex: &str) -> Graph {
    translate(&parse_and_group(regex))
}

fn build_dfa(regex: &str) -> (Graph, &str) {
    (algo::determine(&build_nfa(regex)), regex)
}

//------------------------------------------------------------------
// Parsing (regex string -> HIR)
//------------------------------------------------------------------

#[library_benchmark]
#[benches::one(args = ["hello", "aa*", ".*(?D<1>aa*)"])]
fn parsing(regex: &str) -> Hir {
    black_box(parse(regex))
}

library_benchmark_group!(name = parsing_step, benchmarks = parsing);

//------------------------------------------------------------------
// Translating (HIR -> NFA)
//------------------------------------------------------------------

#[library_benchmark]
#[benches::one(args = ["hello", "aa*", ".*(?D<1>aa*)"], setup = parse_and_group)]
fn translating(hir: Hir) -> Graph {
    black_box(translate(&hir))
}

library_benchmark_group!(name = translating_step, benchmarks = translating);

//------------------------------------------------------------------
// Determinization (NFA -> DFA)
//------------------------------------------------------------------

#[library_benchmark]
#[benches::one(args = ["hello", "aa*", ".*(?D<1>aa*)"], setup = build_nfa)]
fn determinization(nfa: Graph) -> Graph {
    black_box(algo::determine(&nfa))
}

library_benchmark_group!(name = determinization_step, benchmarks = determinization);

//------------------------------------------------------------------
// Code generation (DFA -> TokenStream)
//------------------------------------------------------------------

#[library_benchmark]
#[benches::one(args = ["hello", "aa*", ".*(?D<1>aa*)"], setup = build_dfa)]
fn code_generation(params: (Graph, &str)) -> TokenStream {
    let (dfa, regex) = params;
    let config = Config {
        visibility: quote! { pub(crate) },
        haystack_ty: quote! { str },
        pattern: regex.to_string(),
    };
    black_box(CodeGen::build(config, dfa).generate())
}

library_benchmark_group!(name = codegen_step, benchmarks = code_generation);
