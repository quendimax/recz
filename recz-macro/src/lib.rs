use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use recz_codegen::{CodeGen, Config};
use syn::{Error, LitStr, parse2};

#[proc_macro]
pub fn __re(body: TokenStream) -> TokenStream {
    re_impl(body.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn re_impl(body: TokenStream2) -> syn::Result<TokenStream2> {
    let literal = parse2::<LitStr>(body)?;

    let config = Config {
        visibility: quote! { pub(crate) },
        haystack_ty: quote! { str },
        pattern: literal.clone(),
    };

    let generator = CodeGen::build(config).map_err(|err| Error::new(literal.span(), err))?;
    let code = generator.generate();

    Ok(code)
}
