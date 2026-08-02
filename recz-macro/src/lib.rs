use proc_macro2::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn re(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = parse_macro_input!(body as LitStr);
    re_impl(literal).into()
}

fn re_impl(literal: LitStr) -> TokenStream {
    literal.value();
    quote! { #literal }
}
