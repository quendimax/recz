use clap::{Parser, ValueEnum};
use core::fmt;
use miette::Report;
use owo_colors::{OwoColorize, Stream};
use recz_adt::Legible;
use recz_graph::{Graph, Translator, algo};
use recz_syntax::codec::{AsciiCodec, Utf8Codec};
use recz_syntax::{Error as SyntaxError, Parser as ReParser};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Codec {
    #[default]
    /// ASCII codec
    Ascii,

    /// UTF-8 codec
    Utf8,
}

/// A tool to facilitate development of `recz` crate. It allows you to see inner
/// representation of regex patterns: HIR, NFA, DFA, etc.
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// the regex pattern to analyze
    regex: String,

    /// print HIR
    #[arg(long)]
    print_hir: bool,

    /// print NFA
    #[arg(long)]
    print_nfa: bool,

    /// print DFA
    #[arg(long)]
    print_dfa: bool,

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

    let hir = match cli.codec {
        Codec::Ascii => ReParser::new(AsciiCodec).parse(&cli.regex),
        Codec::Utf8 => ReParser::new(Utf8Codec).parse(&cli.regex),
    }
    .map_err(|e| error_report(&cli.regex, *e))?;

    if cli.print_hir {
        println!("--- HIR ---------------------------------------------------");
        println!();
        println!("{}", dysplay(&hir));
        println!();
    }

    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    let start_node = nfa.start_node();
    let final_node = nfa.node();
    final_node.finalize();
    tr.translate(&hir, start_node, final_node);

    if cli.print_nfa {
        println!("--- NFA ---------------------------------------------------");
        println!();
        println!("{}", dysplay(&nfa));
        println!();
    }

    let dfa = algo::determine(nfa);
    if cli.print_dfa {
        println!("--- DFA ---------------------------------------------------");
        println!();
        println!("{}", dysplay(&dfa));
        println!();
    }

    Ok(())
}
