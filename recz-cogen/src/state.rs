use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use recz_adt::Map;
use recz_graph::{Graph, Node};

pub struct EnumState<'a> {
    name: String,
    graph: &'a Graph,
    dict: Map<Node<'a>, String>,
}

impl<'a> EnumState<'a> {
    pub fn new(name: impl Into<String>, graph: &'a Graph) -> Self {
        Self {
            name: name.into(),
            graph,
            dict: Map::default(),
        }
    }

    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    pub fn state_name(&self, node: Node<'a>) -> &str {
        if let Some(name) = self.dict.get(&node) {
            name
        } else {
            assert!(node.belongs_to(self.graph));
            self.dict.insert(node, format!("{}", node)).unwrap()
        }
    }

    pub fn generate(&self) -> TokenStream {
        let name = self.ident();
        let states = self.dict.values().map(|name| format_ident!("{}", name));
        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum #name {
                #( #states ),*
            }
        }
    }
}
