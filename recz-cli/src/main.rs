use anyhow::Result;
use argh::FromArgs;
use recz_adt::Legible;
use recz_graph::{Graph, Translator, algo};
use recz_syntax::{Parser, codec::Utf8Codec};
use supports_color::Stream;

/// A tool to facilitate development of `recz` crate. It allows you to see inner
/// representation of regex patterns: HIR, NFA, DFA, etc.
#[derive(FromArgs)]
struct Cli {
    /// the regex pattern to analyze
    #[argh(positional)]
    regex: String,

    /// print HIR to stdout
    #[argh(switch)]
    print_hir: bool,

    /// print NFA to stdout
    #[argh(switch)]
    print_nfa: bool,

    /// print DFA to stdout
    #[argh(switch)]
    print_dfa: bool,
}

fn dysplay<T: Legible>(item: &T) -> impl std::fmt::Display {
    struct DisplayWrapper<'a, T>(&'a T);

    impl<'a, T: Legible> std::fmt::Display for DisplayWrapper<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if supports_color::on(Stream::Stdout).is_some() {
                self.0.colored().fmt(f)
            } else {
                self.0.legible().fmt(f)
            }
        }
    }

    DisplayWrapper(item)
}

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    let parser = Parser::new(Utf8Codec);
    let hir = parser.parse(&cli.regex)?;

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
