use argh::FromArgs;
use core::fmt;
use miette::{Report, miette};
use owo_colors::{OwoColorize, Stream};
use recz_adt::Legible;
use recz_graph::{Graph, Translator, algo};
use recz_syntax::codec::{AsciiCodec, Utf8Codec};
use recz_syntax::{Error as SyntaxError, Parser};

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

    /// encoding system that regex engine is built for
    #[argh(option, default = "String::from(\"ascii\")")]
    codec: String,
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

fn error_report(source: &str, err: SyntaxError) -> miette::Result<()> {
    let report: Report = err.into();
    Err(report.with_source_code(source.to_owned()))
}

fn main() -> miette::Result<()> {
    let cli: Cli = argh::from_env();

    let hir = match cli.codec.as_str() {
        "ascii" | "ASCII" => {
            let parser = Parser::new(AsciiCodec);
            match parser.parse(&cli.regex) {
                Ok(hir) => hir,
                Err(err) => {
                    return error_report(&cli.regex, *err);
                }
            }
        }
        "utf8" | "utf-8" | "UTF8" | "UTF-8" => {
            let parser = Parser::new(Utf8Codec);
            match parser.parse(&cli.regex) {
                Ok(hir) => hir,
                Err(err) => {
                    return error_report(&cli.regex, *err);
                }
            }
        }
        _ => return Err(miette!("unsupported codec: {}", cli.codec)),
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
