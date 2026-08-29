use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use recz_codegen::{CodeGen, Config};
use recz_graph::{Graph, algo};
use recz_syntax::{Parser, Translator, codec::Utf8Codec};
use syn::{Error, LitStr, parse2};

#[proc_macro]
pub fn __re(body: TokenStream) -> TokenStream {
    re_impl(body.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn re_impl(body: TokenStream2) -> syn::Result<TokenStream2> {
    let literal = parse2::<LitStr>(body)?;
    let re_str = literal.value();
    let re_str = format!(".*(?D<0>{re_str})");

    let parser = Parser::new(Utf8Codec);
    let hir = parser
        .parse(&re_str)
        .map_err(|err| Error::new(literal.span(), err))?;

    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());

    let dfa = algo::determine(nfa);

    let config = Config {
        visibility: quote! { pub(crate) },
        haystack_ty: quote! { str },
        as_fn: quote! { as_str },
        pattern: literal,
    };

    let generator = CodeGen::new(config);
    let code = generator.generate(&dfa);

    Ok(code)
}
