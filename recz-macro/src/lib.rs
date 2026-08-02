use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn re(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = parse_macro_input!(body as LitStr);
    recz_cogen::regex_instance(&literal.value()).into()
}
