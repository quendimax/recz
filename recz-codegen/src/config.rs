use proc_macro2::TokenStream;

pub struct Config {
    pub visibility: TokenStream,
    pub haystack_ty: TokenStream,
    pub pattern: String,
}
