use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use recz_graph::{Graph, Translator, algo};
use recz_syntax::{Parser, codec::Utf8Codec};
use syn::{Error, LitStr, parse_macro_input};

#[proc_macro]
pub fn re(body: TokenStream) -> TokenStream {
    re_impl(parse_macro_input!(body as LitStr))
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn re_impl(literal: LitStr) -> syn::Result<TokenStream2> {
    let re_str = literal.value();
    let re_str = format!(".*(?<0>{re_str})");

    let parser = Parser::new(Utf8Codec);
    let hir = parser
        .parse(&re_str)
        .map_err(|err| Error::new(literal.span(), err))?;

    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());

    let dfa = algo::determine(nfa);

    Ok(recz_cogen::regex_instance(dfa))
}
