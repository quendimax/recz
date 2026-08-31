use crate::Config;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use recz_adt::Set;
use recz_graph::{CaptureLabel, Graph, Node, algo};
use recz_syntax::{Hir, Parser, Result, Translator, codec::Utf8Codec};

pub struct CodeGen {
    config: Config,
    labels: Vec<CaptureLabel>,
    mtch_dfa: Graph,
}

impl CodeGen {
    pub fn build(config: Config) -> Result<Self> {
        let parser = Parser::new(Utf8Codec);
        let hir = parser.parse(&config.pattern.value())?;
        let hir = Hir::group(0u32, hir);
        let nfa = Graph::new();
        let mut tr = Translator::new(&nfa);
        tr.translate(&hir, nfa.start_node(), nfa.node().finalize());
        let mtch_dfa = algo::determine(&nfa);

        let labels: Vec<_> = nfa.groups().map(|gr| gr.label()).collect();
        assert_ne!(labels.len(), 0);

        Ok(Self {
            config,
            labels,
            mtch_dfa,
        })
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

        let mut mtch_gen = MatchGenerator::new(&self.mtch_dfa);
        mtch_gen.run();

        let mtch_states = mtch_gen.states();
        let start_node = self.mtch_dfa.start_node();
        let start_state_ident = format_ident!("{start_node}");
        let match_branches = mtch_gen.branches();

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
                        if range.end != usize::MAX {
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
                        self.match_impl(haystack, &mut m);
                        m
                    }

                    #[inline]
                    #vis fn find<'h>(&self, haystack: &'h #hay_ty) -> Option<Match<'h>> {
                        unimplemented!()
                    }
                }

                impl Regex {
                    fn match_impl<'h>(&self, haystack: &'h #hay_ty, m: &mut Option<Match<'h>>) {
                        const INVALID_RANGE: Range<usize> = Range {
                            start: usize::MAX,
                            end: usize::MAX,
                        };

                        #[allow(non_camel_case_types)]
                        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                        enum stt {
                            #(#mtch_states,)*
                        }

                        let source = AsRef::<[u8]>::as_ref(haystack);
                        let mut ranges = [INVALID_RANGE; #labels_len];
                        let mut pos = 0usize;
                        let mut curr_state = stt::#start_state_ident;

                        'main: loop {
                            match curr_state {
                                #(#match_branches)*
                            }
                            pos += 1;
                        }
                    }
                }
            }

            adhoc::Regex
        })
    }
}

struct MatchGenerator<'d> {
    dfa: &'d Graph,
    states: Set<syn::Ident>,
    branches: Vec<TokenStream>,
    stack: Vec<Node<'d>>,
}

impl<'d> MatchGenerator<'d> {
    fn new(dfa: &'d Graph) -> Self {
        Self {
            dfa,
            states: Set::default(),
            branches: Vec::default(),
            stack: Vec::with_capacity(dfa.node_count()),
        }
    }

    fn run(&mut self) {
        let visited = Set::default();

        self.stack.push(self.dfa.start_node());
        while let Some(node) = self.stack.pop() {
            dbg!(node);
            visited.insert(node);

            let state_ident = format_ident!("{node}");
            self.states.insert(state_ident.clone());

            if node.is_epilogue() {
                self.branches.push(quote! {
                    stt::#state_ident => {
                        *m = Some(Match {
                            hay: haystack,
                            ranges,
                        });
                        break 'main;
                    }
                });
                continue;
            }

            let mut sub_branches = Vec::<TokenStream>::with_capacity(node.target_count());
            let mut else_branch = quote! { _ => break 'main };

            for (edge, target) in node.targets() {
                let target_state_ident = format_ident!("{target}");
                if edge.is_epsilon() {
                    else_branch = quote! { _ => curr_state = stt::#target_state_ident };
                } else {
                    let dump_match =
                        if node.is_final() && !target.is_final() && !target.is_epilogue() {
                            Some(quote! {
                                 *m = Some(Match { hay: haystack, ranges })
                            })
                        } else {
                            None
                        };
                    for range in edge.ranges() {
                        let start_byte = range.start();
                        let end_byte = range.last();
                        sub_branches.push(quote! {
                            Some(#start_byte..=#end_byte) => {
                                curr_state = stt::#target_state_ident;
                                #dump_match;
                            }
                        });
                    }
                }
                if !visited.contains(&target) {
                    self.stack.push(target);
                }
            }

            self.branches.push(quote! {
                stt::#state_ident => {
                    match source.get(pos) {
                        #(#sub_branches)*
                        #else_branch,
                    }
                }
            });
        }
    }

    fn states(&self) -> impl Iterator<Item = &syn::Ident> {
        self.states.iter()
    }

    fn branches(&self) -> impl Iterator<Item = &TokenStream> {
        self.branches.iter()
    }
}

// Possible optimizations:
//
// 1. If final state doesnt have output edges exept for epsilone edge to
// epilogue without tags, it can be removed
//
// 2. If node has only one outgoing edge with one symbol, it can be combined
// with the next node.
