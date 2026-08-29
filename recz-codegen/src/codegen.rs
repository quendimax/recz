use crate::Config;
use proc_macro2::TokenStream;
use quote::quote;
use recz_graph::Graph;

pub struct CodeGen {
    config: Config,
}

impl CodeGen {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate(&self, _dfa: &Graph) -> TokenStream {
        let vis = &self.config.visibility;
        let hay_ty = &self.config.haystack_ty;
        let as_fn = &self.config.as_fn;
        let pattern = &self.config.pattern;

        quote!({
            mod adhoc {
                use ::core::option::Option;
                use ::core::range::Range;
                use ::recz::Label;

                //------------------------------------------------------------------
                // Capture
                //------------------------------------------------------------------
                #[derive(Debug, Clone, PartialEq, Eq)]
                #vis struct Capture<'h> {
                    capture: &'h #hay_ty,
                    start: usize
                }

                impl<'h> Capture<'h> {
                    #[inline]
                    #vis fn #as_fn(&self) -> &'h #hay_ty {
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
                    #vis fn span(&self) -> Range<usize> {
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
                    #vis fn #as_fn(&self) -> &'h #hay_ty {
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
                    #vis fn span(&self) -> Range<usize> {
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
                            Label::Num(n) => self.capture_by_num(n),
                            Label::Str(s) => self.capture_by_str(s),
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

                    #vis fn r#match<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        None
                    }

                    #vis fn search<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        None
                    }
                }
            }

            adhoc::Regex
        })
    }
}
