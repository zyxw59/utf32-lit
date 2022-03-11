use quote::quote;
use syn::{
    parse_macro_input, parse_quote_spanned,
    visit_mut::{self, VisitMut},
    Expr, Lit,
};

/// Converts `&str` literals to `&[char]`.
///
/// ```rust
/// use utf32_lit::utf32;
/// let s = utf32!("æbc");
/// assert_eq!(s, &['æ', 'b', 'c']);
///
/// let s_array = utf32!(&["foo", "bar", "baz"]);
/// assert_eq!(s_array, &[&['f', 'o', 'o'], &['b', 'a', 'r'], &['b', 'a', 'z']]);
/// ```
#[proc_macro]
pub fn utf32(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expr = parse_macro_input!(input as Expr);
    Utf32Replace.visit_expr_mut(&mut expr);
    quote!(#expr).into()
}

struct Utf32Replace;

impl VisitMut for Utf32Replace {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(expr) = &node {
            if let Lit::Str(s) = &expr.lit {
                let string = s.value();
                let chars = string.chars();
                *node = parse_quote_spanned!(s.span()=> &[#(#chars),*]);
            }
        }
        visit_mut::visit_expr_mut(self, node);
    }
}
