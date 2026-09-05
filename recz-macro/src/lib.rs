use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use recz_codegen::{CodeGen, Config};
use recz_graph::{Graph, algo};
use recz_syntax::{Hir, Parser, Translator, codec::Utf8Codec};
use syn::{Error, LitStr, parse2};

#[proc_macro]
pub fn __re(body: TokenStream) -> TokenStream {
    re_impl(body.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn re_impl(body: TokenStream2) -> syn::Result<TokenStream2> {
    let literal = parse2::<LitStr>(body)?;

    let parser = Parser::new(Utf8Codec);
    let hir = parser
        .parse(&literal.value())
        .map_err(|err| Error::new(literal.span(), err))?;

    let hir = Hir::group(0u32, hir);
    let nfa = Graph::new();
    let mut tr = Translator::new(&nfa);
    tr.translate(&hir, nfa.start_node(), nfa.node().finalize());

    let dfa = algo::determine(&nfa);

    let config = Config {
        visibility: quote! { pub(crate) },
        haystack_ty: quote! { str },
        pattern: literal.value(),
    };

    let generator = CodeGen::build(config, dfa);
    let code = generator.generate();
    let code = quote!({
        mod adhoc {
            #code
        }
        adhoc::Regex
    });

    Ok(code)
}
