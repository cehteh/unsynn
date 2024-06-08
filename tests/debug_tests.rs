#![allow(clippy::items_after_statements)]
#![cfg(feature = "impl_debug")]
use unsynn::*;

// Debug formats are not stable, we're stripping whitespace from strings to make the tests
// more reliable. Only a few tests are included here to check it works, when it breaks please
// investigate and PR
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
                 Ident{sym:foo},
                 Ident{sym:bar}
             ],
         then:OnePunct<';'>
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
                 Delimited<proc_macro2::Ident, unsynn::punct::OnePunct<';'>>(
                     Ident{sym:foo},Some(OnePunct<';'>)
                 ),
                 Delimited<proc_macro2::Ident,unsynn::punct::OnePunct<';'>>(
                     Ident{sym:bar},Some(OnePunct<';'>)
                 ),
                 Delimited<proc_macro2::Ident,unsynn::punct::OnePunct<';'>>(
                     Ident{sym:baz},None
                 )
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
             GroupContaining<proc_macro2::Ident>{
                 delimiter:Brace,
                 content:Ident{sym:foo}
             }
         )"
        .strip_whitespace()
    );
}
