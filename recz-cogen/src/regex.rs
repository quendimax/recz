mod capture;
pub use capture::Capture;

mod r#match;
pub use r#match::Match;

mod state;
pub use state::EnumState;

mod automaton;
pub use automaton::Automaton;

use proc_macro2::TokenStream;
use quote::quote;
use recz_graph::Graph;

pub fn re_impl(dfa: Graph) -> TokenStream {
    let vis = quote! { pub(crate) };
    let capture_def = Capture::new(vis.clone()).generate();
    let match_def = Match::new(vis.clone()).generate();
    let state_def = EnumState::new("State", &dfa).generate();
    let regex = Automaton::new(&dfa, vis.clone());

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
