use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use recz_graph::Graph;

pub struct Automaton<'a> {
    _graph: &'a Graph,
    name: String,
    vis: TokenStream,
}

impl<'a> Automaton<'a> {
    pub fn new(_graph: &'a Graph, vis: TokenStream) -> Self {
        Self {
            _graph,
            name: "Regex".into(),
            vis,
        }
    }

    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    fn generate_test(&self) -> TokenStream {
        quote! {
            fn test<'h>(&self, hay: &'h str) -> Option<Match<'h>> {
                let m = Match::new();
                None
            }
        }
    }

    pub fn generate(&self) -> TokenStream {
        let name = self.ident();
        let vis = &self.vis;
        let test_def = self.generate_test();
        quote! {
            #vis struct #name;

            impl #name {
                #vis #test_def
            }
        }
    }
}
