mod state;
pub use state::EnumState;

mod regex;
pub use regex::Regex;

use proc_macro2::TokenStream;
use quote::quote;
use recz_graph::Graph;

pub fn re_impl(dfa: Graph) -> TokenStream {
    let state = EnumState::new("State", &dfa);
    let regex = Regex::new(&dfa);

    let state_toks = state.generate();
    let regex_impl_toks = regex.generate();
    let regex_instance_toks = regex.ident();
    quote! {{
        mod __adhoc {
            #state_toks
            #regex_impl_toks
        }
        __adhoc::#regex_instance_toks
    }}
}
