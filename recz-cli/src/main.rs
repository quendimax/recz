use bat::PrettyPrinter;
use clap::{Parser as ClapParser, ValueEnum};
use core::fmt;
use miette::Report;
use owo_colors::{OwoColorize, Stream};
use quote::quote;
use recz_adt::Legible;
use recz_codegen::{CodeGen, Config};
use recz_graph::{Graph, algo};
use recz_syntax::codec::{AsciiCodec, Latin1Codec, Utf8Codec};
use recz_syntax::{Error as SyntaxError, Parser, Translator};
use std::time::Instant;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Codec {
    #[default]
    /// ASCII codec
    Ascii,

    /// Latin-1 codec
    Latin1,

    /// UTF-8 codec
    Utf8,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PrintMode {
    /// Print HIR
    Hir,
    /// Print NFA
    Nfa,
    /// Print DFA
    Dfa,
    /// Print Code
    Code,
    /// Print all transformation steps
    All,
}

/// A tool to facilitate development of `recz` crate. It allows you to see inner
/// representation of regex patterns: HIR, NFA, DFA, etc.
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// the regex pattern to analyze
    regex: String,

    /// what inner representation to print
    #[arg(short, long, value_enum)]
    print: Vec<PrintMode>,

    /// encoding system that regex engine is built for
    #[arg(short, long, value_enum, default_value = "ascii")]
    codec: Codec,
}

fn dysplay<T: fmt::Display + Legible>(item: &T) -> impl fmt::Display {
    struct DisplayWrapper<'a, T>(&'a T);

    impl<'a, T: fmt::Display + Legible> fmt::Display for DisplayWrapper<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0
                .if_supports_color(Stream::Stdout, |v| v.colored())
                .fmt(f)
        }
    }

    DisplayWrapper(item)
}

fn error_report(source: &str, err: SyntaxError) -> Report {
    let report: Report = err.into();
    report.with_source_code(source.to_owned())
}

fn main() -> miette::Result<()> {
    let cli: Cli = Cli::parse();
    let print_all = cli.print.contains(&PrintMode::All);

    let hir_start = Instant::now();
    let hir = match cli.codec {
        Codec::Ascii => Parser::new(AsciiCodec).parse(&cli.regex),
        Codec::Latin1 => Parser::new(Latin1Codec).parse(&cli.regex),
        Codec::Utf8 => Parser::new(Utf8Codec).parse(&cli.regex),
    }
    .map_err(|e| error_report(&cli.regex, *e))?;
    let hir_duration = hir_start.elapsed();

    if cli.print.contains(&PrintMode::Hir) || print_all {
        println!("--- HIR ----------------------------- {hir_duration:?} --------");
        println!();
        println!("{}", dysplay(&hir));
        println!();
    }

    let nfa_start = Instant::now();
    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());
    let nfa_duration = nfa_start.elapsed();

    if cli.print.contains(&PrintMode::Nfa) || print_all {
        println!("--- NFA ----------------------------- {nfa_duration:?} --------");
        println!();
        println!("{}", dysplay(&nfa));
        println!();
    }

    let dfa_start = Instant::now();
    let dfa = algo::determine(&nfa);
    drop(nfa);
    let dfa_duration = dfa_start.elapsed();

    if cli.print.contains(&PrintMode::Dfa) || print_all {
        println!("--- DFA ----------------------------- {dfa_duration:?} --------");
        println!();
        println!("{}", dysplay(&dfa));
        println!();
    }

    let code_start = Instant::now();

    let config = Config {
        visibility: quote! { pub(crate) },
        haystack_ty: quote! { str },
        pattern: cli.regex.clone(),
    };
    let codegen = CodeGen::build(config, dfa);
    let code_stream = codegen.generate();
    let code_duration = code_start.elapsed();

    if cli.print.contains(&PrintMode::Code) || print_all {
        let code_file: syn::File = syn::parse2(code_stream).unwrap();
        let code = prettyplease::unparse(&code_file);
        println!("--- Code ---------------------------- {code_duration:?} --------");
        println!();
        PrettyPrinter::new()
            .input_from_bytes(code.as_bytes())
            .language("rust")
            .theme("ansi")
            .tab_width(Some(4))
            .print()
            .unwrap();
        println!();
    }

    let total_duration = hir_duration + nfa_duration + dfa_duration + code_duration;
    println!("Elapsed time for building");
    println!("- HIR:   {hir_duration:?}");
    println!("- NFA:   {nfa_duration:?}");
    println!("- DFA:   {dfa_duration:?}");
    println!("- Code:  {code_duration:?}");
    println!("> Total: {total_duration:?}");

    Ok(())
}
