use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use recz_graph::Graph;

pub struct Regex<'a> {
    _graph: &'a Graph,
    name: String,
}

impl<'a> Regex<'a> {
    pub fn new(_graph: &'a Graph) -> Self {
        Self {
            _graph,
            name: "Regex".into(),
        }
    }

    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    fn generate_match(&self) -> TokenStream {
        quote! {
            pub fn match_str(&self, hay: &str) -> bool {
                true
            }
        }
    }

    pub fn generate(&self) -> TokenStream {
        let name = self.ident();
        let match_toks = self.generate_match();
        quote! {
            pub struct #name;
            impl #name {
                #match_toks
            }
        }
    }
}
