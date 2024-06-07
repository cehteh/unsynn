#![allow(clippy::items_after_statements)]
#![cfg(feature = "impl_display")]
use unsynn::*;

#[test]
fn test_lazy_vec() {
    let mut token_iter = quote::quote! {foo bar ;}.into_iter();

    type Example = LazyVec<TokenTree, Semicolon>;

    let example = Example::parse(&mut token_iter).unwrap();

    assert_eq!(format!("{example}"), "foo bar ;");
}

#[test]
fn test_repeats() {
    let mut token_iter = quote::quote! {foo ; bar ; baz }.into_iter();

    type Example = Exactly<3, Ident, Semicolon>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(format!("{example}"), "foo;bar;baz ");
}

#[test]
fn test_group_containing() {
    let mut token_iter = quote::quote! { { foo } }.into_iter();

    type Example = BraceGroupContaining<Ident>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(format!("{example}"), "{ foo }");
}

#[test]
fn test_unsynn_macro() {
    unsynn! {
        struct Tuple(Ident,Ident);
    }

    let mut token_iter = quote::quote! { foo bar }.into_iter();

    let example = Tuple::parse(&mut token_iter).unwrap();
    assert_eq!(format!("{example}"), "foo bar ");
}
