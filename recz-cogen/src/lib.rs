mod capture;
pub use capture::Capture;

mod r#match;
pub use r#match::Match;

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

    let vis = quote! { pub(crate) };
    let capture_def = Capture::new(vis.clone()).generate();
    let match_def = Match::new(vis.clone()).generate();
    let state_def = state.generate();
    let regex_def = regex.generate();
    let regex_inst = regex.ident();

    quote!({
        #capture_def
        #match_def
        #state_def
        #regex_def
        #regex_inst
    })
}
