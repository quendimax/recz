use proc_macro2::TokenStream;
use syn::LitStr;

pub struct Config {
    pub visibility: TokenStream,
    pub haystack_ty: TokenStream,
    pub as_fn: TokenStream,
    pub pattern: LitStr,
}
