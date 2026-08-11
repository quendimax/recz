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
                capture: &'h str,
                start: usize,
            }

            impl<'h> Capture<'h> {
                #vis fn new(label: u32, capture: &'h str, start: usize) -> Self {
                    Self {
                        label,
                        capture,
                        start,
                    }
                }

                #[inline]
                #vis fn label(&self) -> u32 {
                    self.label
                }

                #[inline]
                #vis fn capture(&self) -> &'h str {
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
                #vis fn span(&self) -> ::core::range::Range<usize> {
                    ::core::range::Range {
                        start: self.start(),
                        end: self.end(),
                    }
                }
            }
        }
    }
}
