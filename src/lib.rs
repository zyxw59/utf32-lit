use quote::quote_spanned;
use syn::{parse_macro_input, Error, Lit};

/// Converts a `&str` literal to `&[char]`.
///
/// ```rust
/// use utf32_lit::utf32;
/// let s = utf32!("æbc");
/// assert_eq!(s, &['æ', 'b', 'c']);
/// ```
#[proc_macro]
pub fn utf32(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as Lit) {
        Lit::Str(s) => {
            let string = s.value();
            let chars = string.chars();
            quote_spanned!(s.span()=> &[#(#chars),*])
        }
        other => {
            let message = "expected string literal";
            Error::new_spanned(other, message).to_compile_error()
        }
    }
    .into()
}
