use proc_macro2::TokenStream;
use quote::quote;

pub fn regex_instance(regex: &str) -> TokenStream {
    quote! {#regex}
}
