use proc_macro2::TokenStream;
use quote::quote;
use recz_adt::Map;

pub struct Match {
    vis: TokenStream,
    lbl_to_idx: Map<u32, usize>,
}

impl Match {
    pub fn new(vis: TokenStream) -> Self {
        Self {
            vis,
            lbl_to_idx: Map::default(),
        }
    }

    fn generate_fn_group(&self) -> TokenStream {
        let match_branches = self.lbl_to_idx.iter().map(|(label, index)| {
            quote! {
                #label => self.groups[#index].as_ref(),
            }
        });
        quote! {
            fn group(&self, label: u32) -> Option<&Capture<'h>> {
                match label {
                    #(#match_branches)*
                    _ => None,
                }
            }
        }
    }

    fn generate_fn_groups(&self) -> TokenStream {
        quote! {
            fn groups(&self) -> impl Iterator<Item = &Capture<'h>> {
                self.groups.iter().filter_map(|g| g.as_ref())
            }
        }
    }

    pub fn generate(&self) -> TokenStream {
        let vis = &self.vis;
        let group_count = self.lbl_to_idx.len();

        let fn_group = self.generate_fn_group();
        let fn_groups = self.generate_fn_groups();

        quote! {
            #vis struct Match<'h> {
                groups: [::std::option::Option<Capture<'h>>; #group_count]
            }

            impl<'h> Match<'h> {
                #[inline]
                fn new() -> Self {
                    Self {
                        groups: [::std::option::Option::None; #group_count],
                    }
                }

                fn init_groups(&mut self, hay: &'h str, groups: [(u32, usize, usize); #group_count]) {
                    for (i, (label, start, end)) in groups.iter().enumerate() {
                        self.groups[i] = ::std::option::Option::Some(Capture::new(
                            *label,
                            hay,
                            (*start..*end).into()
                        ));
                    }
                }

                #[inline]
                #vis #fn_group

                #[inline]
                #vis #fn_groups
            }

            impl<'h> ::core::ops::Deref for Match<'h> {
                type Target = Capture<'h>;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.groups[0].as_ref().unwrap()
                }
            }

            impl<'h> ::core::convert::AsRef<Capture<'h>> for Match<'h> {
                #[inline]
                fn as_ref(&self) -> &Capture<'h> {
                    ::core::ops::Deref::deref(self)
                }
            }
        }
    }
}
