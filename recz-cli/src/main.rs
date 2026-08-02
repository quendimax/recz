use argh::FromArgs;
use core::fmt;
use miette::{Diagnostic, SourceSpan};
use owo_colors::{OwoColorize, Stream};
use recz_adt::Legible;
use recz_graph::{Graph, Translator, algo};
use recz_syntax::{Error as SyntaxError, Parser, codec::Utf8Codec};
use thiserror::Error;

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

fn error_report(source: &str, err: Box<SyntaxError>) -> miette::Result<()> {
    #[derive(Error, Debug, Diagnostic)]
    #[error("invalid regular expression format")]
    struct MyBad {
        msg: String,

        #[source_code]
        src: String,

        #[label("{msg}")]
        span: SourceSpan,
    }
    Err(MyBad {
        msg: err.to_string(),
        src: source.to_owned(),
        span: err.error_span().into(),
    })?;
    Ok(())
}

fn main() -> miette::Result<()> {
    let cli: Cli = argh::from_env();
    let parser = Parser::new(Utf8Codec);
    let hir = match parser.parse(&cli.regex) {
        Ok(hir) => hir,
        Err(err) => {
            error_report(&cli.regex, err)?;
            return Ok(());
        }
    };

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
