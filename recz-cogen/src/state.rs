use proc_macro2::TokenStream;
use quote::quote;
use recz_adt::Map;
use recz_graph::{Graph, Node};

pub struct EnumState<'a> {
    name: String,
    dict: Map<Node<'a>, String>,
}

impl<'a> EnumState<'a> {
    pub fn new(dfa: &'a Graph) -> Self {
        let dict = Map::default();
        dict.insert(dfa.start_node(), "Start".into());
        for node in dfa.nodes() {
            if node.is_final() {
                dict.insert(node, "Exit".into());
                for (source, ed) in node.sources() {
                    debug_assert!(ed.is_epsilon());
                    dict.insert(source, format!("Final{}", source.nid()));
                }
            } else {
                dict.insert(node, format!("State{}", node.nid()));
            }
        }
        Self {
            name: "State".into(),
            dict,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state_name(&self, node: Node<'a>) -> &str {
        &self.dict[&node]
    }

    pub fn generate(&self) -> TokenStream {
        let name = self.name();
        let states = self.dict.values();
        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum #name {
                #(#states),*
            }
        }
    }
}
