use argh::FromArgs;

/// dev tools for analyzing `recz`'s inner NFA/DFA graphs.
#[derive(FromArgs)]
struct Cli {
    /// the regex pattern to analyze
    #[argh(positional)]
    regex: String,
}

use anyhow::Result;
use recz_graph::{Graph, Translator};
use recz_syntax::{Parser, codec::Utf8Codec};

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    let parser = Parser::new(Utf8Codec);

    let hir = parser.parse(&cli.regex)?;
    println!("{hir}");

    let gr = Graph::new();
    let mut tr = Translator::new(&gr);

    tr.translate(&hir, gr.start_node(), gr.node());

    println!("{gr}");

    Ok(())
}
