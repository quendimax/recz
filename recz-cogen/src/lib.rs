mod state;
pub use state::EnumState;

use proc_macro2::TokenStream;
use quote::quote;
use recz_graph::Graph;

pub fn regex_instance(_dfa: Graph) -> TokenStream {
    quote! { "" }
}
