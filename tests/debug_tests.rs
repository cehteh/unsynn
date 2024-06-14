#![allow(clippy::items_after_statements)]
#![cfg(feature = "impl_debug")]
use unsynn::*;

// Debug formats are not stable, we're stripping whitespace from strings to make the tests
// more reliable and readable. Only a few tests are included here to check it works, when it
// breaks please investigate and PR. Note: this is a hack as it joins text/tokens together.
trait StripWhitespace {
    fn strip_whitespace(self) -> String;
}

impl StripWhitespace for String {
    fn strip_whitespace(self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

impl StripWhitespace for &str {
    fn strip_whitespace(self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

#[test]
fn test_lazy_vec() {
    let mut token_iter = quote::quote! {foo bar ; baz ;}.into_iter();

    type Example = LazyVec<TokenTree, Semicolon>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(
        format!("{example:?}").strip_whitespace(),
        "LazyVec<proc_macro2::TokenTree,unsynn::punct::OnePunct<';'>>{
             vec:[
                 Ident{sym: foo},
                 Ident{sym: bar}
             ],
         terminator: OnePunct<';'>
         }"
        .strip_whitespace()
    );
}

#[test]
fn test_repeats() {
    let mut token_iter = quote::quote! {foo ; bar ; baz }.into_iter();

    type Example = Exactly<3, Ident, Semicolon>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(
        format!("{example:?}").strip_whitespace(),
        "Repeats<3,3,proc_macro2::Ident,unsynn::punct::OnePunct<';'>>(
             [
                 Delimited<proc_macro2::Ident, unsynn::punct::OnePunct<';'>>{
                     value: Ident{sym:foo},
                     delimiter: Some(OnePunct<';'>)
                 },
                 Delimited<proc_macro2::Ident,unsynn::punct::OnePunct<';'>>{
                     value: Ident{sym:bar},
                     delimiter: Some(OnePunct<';'>)
                 },
                 Delimited<proc_macro2::Ident,unsynn::punct::OnePunct<';'>>{
                     value: Ident{sym:baz},
                     delimiter: None
                 }
             ]
         )"
        .strip_whitespace()
    );
}

#[test]
fn test_group_containing() {
    let mut token_iter = quote::quote! { { foo } }.into_iter();

    type Example = BraceGroupContaining<Ident>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(
        format!("{example:?}").strip_whitespace(),
        "BraceGroupContaining<proc_macro2::Ident>(
                 Ident{sym:foo}
         )"
        .strip_whitespace()
    );
}
