use crate::Config;
use proc_macro2::TokenStream;
use quote::quote;
use recz_graph::{Graph, algo};
use recz_syntax::{Hir, Translator};

pub struct CodeGen {
    hir: Hir,
    config: Config,
}

impl CodeGen {
    pub fn new(hir: Hir, config: Config) -> Self {
        Self {
            hir: Hir::group(0u32, hir),
            config,
        }
    }

    fn generate_match_impl(&self) -> TokenStream {
        let nfa = Graph::new();
        let mut tr = Translator::new(&nfa);
        tr.translate(&self.hir, nfa.start_node(), nfa.node().finalize());
        let _dfa = algo::determine(&nfa);

        quote! {
            fn match_impl<'h>(&self, haystack: &'h [u8], m: &mut Option<Match<'h>>) {

            }
        }
    }

    pub fn generate(&self) -> TokenStream {
        let vis = &self.config.visibility;
        let hay_ty = &self.config.haystack_ty;
        let pattern = &self.config.pattern;

        let as_str_fn = if self.config.haystack_ty.to_string() == "str" {
            quote! { as_str }
        } else {
            quote! { as_bytes }
        };

        let match_impl_fn = self.generate_match_impl();

        quote!({
            mod adhoc {
                use ::core::convert::AsRef;
                use ::core::option::Option;
                use ::core::range::Range;
                use ::recz::Label;

                //------------------------------------------------------------------
                // Capture
                //------------------------------------------------------------------

                #[derive(Debug, Clone, PartialEq, Eq)]
                #vis struct Capture<'h> {
                    capture: &'h #hay_ty,
                    start: usize,
                }

                impl<'h> Capture<'h> {
                    #[inline]
                    #vis fn #as_str_fn(&self) -> &'h #hay_ty {
                        self.capture
                    }

                    #[inline]
                    #vis fn start(&self) -> usize {
                        self.start
                    }

                    #[inline]
                    #vis fn end(&self) -> usize {
                        self.start + self.capture.len()
                    }

                    #[inline]
                    #vis fn range(&self) -> Range<usize> {
                        Range {
                            start: self.start(),
                            end: self.end(),
                        }
                    }

                    #[inline]
                    #vis fn len(&self) -> usize {
                        self.capture.len()
                    }

                    #[inline]
                    #vis fn is_empty(&self) -> bool {
                        self.capture.is_empty()
                    }
                }

                //------------------------------------------------------------------
                // Match
                //------------------------------------------------------------------

                #[derive(Debug, Clone, PartialEq, Eq)]
                #vis struct Match<'h> {
                    hay: &'h #hay_ty,
                }

                /// Implementation of Capture API.
                impl<'h> Match<'h> {
                    #[inline]
                    #vis fn #as_str_fn(&self) -> &'h #hay_ty {
                        unimplemented!()
                    }

                    #[inline]
                    #vis fn start(&self) -> usize {
                        unimplemented!()
                    }

                    #[inline]
                    #vis fn end(&self) -> usize {
                        unimplemented!()
                    }

                    #[inline]
                    #vis fn range(&self) -> Range<usize> {
                        unimplemented!()
                    }

                    #[inline]
                    #vis fn len(&self) -> usize {
                        unimplemented!()
                    }

                    #[inline]
                    #vis fn is_empty(&self) -> bool {
                        unimplemented!()
                    }
                }

                impl<'h> Match<'h> {
                    #[inline]
                    #vis fn haystack(&self) -> &'h #hay_ty {
                        self.hay
                    }

                    #[inline]
                    #vis fn capture<'a>(&self, label: impl Into<Label<'a>>) -> Option<Capture<'h>> {
                        match label.into() {
                            Label::Str("") => None,
                            Label::Str(s) => self.capture_by_str(s),
                            Label::Num(n) => self.capture_by_num(n),
                        }
                    }

                    #vis fn capture_by_num(&self, label: u32) -> Option<Capture<'h>> {
                        None
                    }

                    #vis fn capture_by_str(&self, label: &str) -> Option<Capture<'h>> {
                        None
                    }
                }

                //------------------------------------------------------------------
                // Regex
                //------------------------------------------------------------------

                #[derive(Debug, Clone, PartialEq, Eq)]
                #vis struct Regex;

                impl Regex {
                    #[inline]
                    #vis const fn pattern(&self) -> &'static str {
                        #pattern
                    }

                    #[inline]
                    #vis const fn capture_labels(&self) -> &'static [Label<'static>] {
                        &[Label::Num(0)]
                    }

                    #vis fn mtch<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        let mut m = None;
                        self.match_impl(AsRef::<[u8]>::as_ref(haystack), &mut m);
                        m
                    }

                    #vis fn find<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        unimplemented!()
                    }

                    #match_impl_fn
                }
            }

            adhoc::Regex
        })
    }
}
