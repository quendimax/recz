use proc_macro2::TokenStream;

pub struct Capture {
    vis: TokenStream,
}

impl Capture {
    pub fn new(vis: TokenStream) -> Self {
        Self { vis }
    }

    pub fn generate(&self) -> TokenStream {
        let vis = &self.vis;

        quote::quote! {
            #vis struct Capture<'h> {
                label: u32,
                hay: &'h str,
                span: ::core::range::Range<usize>,
            }

            impl<'h> Capture<'h> {
                #[inline]
                fn new(label: u32, hay: &'h str, span: ::core::range::Range<usize>) -> Self {
                    Self {
                        label,
                        hay,
                        span
                    }
                }

                #[inline]
                #vis fn label(&self) -> u32 {
                    self.label
                }

                #[inline]
                #vis fn hay(&self) -> &'h str {
                    self.hay
                }

                #[inline]
                #vis fn capture(&self) -> &'h str {
                    &self.hay[self.span]
                }

                #[inline]
                #vis fn start(&self) -> usize {
                    self.span.start
                }

                #[inline]
                #vis fn end(&self) -> usize {
                    self.span.end
                }

                #[inline]
                #vis fn span(&self) -> ::core::range::Range<usize> {
                    self.span
                }
            }
        }
    }
}
