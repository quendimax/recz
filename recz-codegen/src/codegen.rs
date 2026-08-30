use crate::Config;
use proc_macro2::TokenStream;
use quote::quote;
use recz_graph::{CaptureLabel, Graph, algo};
use recz_syntax::{Hir, Parser, Result, Translator, codec::Utf8Codec};

pub struct CodeGen {
    config: Config,
    labels: Vec<CaptureLabel>,
}

impl CodeGen {
    pub fn build(config: Config) -> Result<Self> {
        let parser = Parser::new(Utf8Codec);
        let hir = parser.parse(&config.pattern.value())?;
        let hir = Hir::group(0u32, hir);
        let nfa = Graph::new();
        let mut tr = Translator::new(&nfa);
        tr.translate(&hir, nfa.start_node(), nfa.node().finalize());
        let _dfa = algo::determine(&nfa);

        let labels: Vec<_> = nfa.groups().map(|gr| gr.label()).collect();
        assert_ne!(labels.len(), 0);

        Ok(Self { config, labels })
    }

    fn generate_match_impl(&self) -> TokenStream {
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

        let labels_len = self.labels.len();
        let labels = self.labels.iter().map(|label| match label {
            CaptureLabel::Num(n) => quote! { Label::Num(#n) },
            CaptureLabel::Str(s) => quote! { Label::Str(#s) },
        });

        let match_idx_from_num_branches =
            self.labels.iter().enumerate().filter_map(|(idx, lbl)| {
                if let CaptureLabel::Num(num) = lbl {
                    Some(quote!(#num => #idx))
                } else {
                    None
                }
            });

        let match_idx_from_str_branches =
            self.labels.iter().enumerate().filter_map(|(idx, lbl)| {
                if let CaptureLabel::Str(str) = lbl {
                    Some(quote!(#str => #idx))
                } else {
                    None
                }
            });

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
                    #vis const fn start(&self) -> usize {
                        self.start
                    }

                    #[inline]
                    #vis const fn end(&self) -> usize {
                        self.start + self.capture.len()
                    }

                    #[inline]
                    #vis const fn range(&self) -> Range<usize> {
                        Range {
                            start: self.start(),
                            end: self.end(),
                        }
                    }

                    #[inline]
                    #vis const fn len(&self) -> usize {
                        self.capture.len()
                    }

                    #[inline]
                    #vis const fn is_empty(&self) -> bool {
                        self.capture.is_empty()
                    }
                }

                //------------------------------------------------------------------
                // Match
                //------------------------------------------------------------------

                #[derive(Debug, Clone, PartialEq, Eq)]
                #vis struct Match<'h> {
                    hay: &'h #hay_ty,
                    ranges: [Range<usize>; #labels_len]
                }

                /// Implementation of Capture API.
                impl<'h> Match<'h> {
                    #[inline]
                    #vis fn #as_str_fn(&self) -> &'h #hay_ty {
                        &self.hay[self.range()]
                    }

                    #[inline]
                    #vis const fn start(&self) -> usize {
                        self.ranges[0].start
                    }

                    #[inline]
                    #vis const fn end(&self) -> usize {
                        self.ranges[0].end
                    }

                    #[inline]
                    #vis const fn range(&self) -> Range<usize> {
                        self.ranges[0]
                    }

                    #[inline]
                    #vis const fn len(&self) -> usize {
                        self.end() - self.start()
                    }

                    #[inline]
                    #vis const fn is_empty(&self) -> bool {
                        self.end() == self.start()
                    }
                }

                impl<'h> Match<'h> {
                    #[inline]
                    #vis const fn haystack(&self) -> &'h #hay_ty {
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

                    #[inline]
                    #vis fn capture_by_num(&self, label: u32) -> Option<Capture<'h>> {
                        let idx = match label {
                            #(#match_idx_from_num_branches,)*
                            _ => return None,
                        };
                        self.range_to_capture(idx)
                    }

                    #[inline]
                    #vis fn capture_by_str(&self, label: &str) -> Option<Capture<'h>> {
                        let idx = match label {
                            #(#match_idx_from_str_branches,)*
                            _ => return None,
                        };
                        self.range_to_capture(idx)
                    }
                }

                impl<'h> Match<'h> {
                    #[inline]
                    fn range_to_capture(&self, idx: usize) -> Option<Capture<'h>> {
                        let range = self.ranges[idx];
                        if range.end == usize::MAX {
                            Some(Capture {
                                capture: &self.hay[range],
                                start: range.start,
                            })
                        } else {
                            None
                        }
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
                        &[#(#labels),*]
                    }

                    #[inline]
                    #vis fn mtch<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        let mut m = None;
                        self.match_impl(AsRef::<[u8]>::as_ref(haystack), &mut m);
                        m
                    }

                    #[inline]
                    #vis fn find<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        unimplemented!()
                    }
                }

                impl Regex {
                    #match_impl_fn
                }
            }

            adhoc::Regex
        })
    }
}
