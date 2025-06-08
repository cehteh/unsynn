//! testing `to_tokens`
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_tt_conversion() {
    // some pointless conversions, just to ensure everything works
    let token_stream = " ident foo (bar baz)".to_token_iter().to_token_stream();
    let token_string = token_stream.to_token_iter().tokens_to_string();
    assert_eq!(token_string, "ident foo (bar baz)");
}

#[test]
fn test_tt_selfconvert() {
    // token iter can convert to itself
    let token_iter = " ident foo (bar baz)".to_token_iter();
    let token_iter = token_iter.to_token_iter();
    let token_iter = token_iter.to_token_iter();
    // ...
    assert_tokens_eq!(token_iter, str "ident foo (bar baz)");
}

#[test]
fn test_gen() {
    let mut token_iter = " ident ".to_token_iter();

    let ident = Ident::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    ident.to_tokens(&mut out);
    assert_eq!(out.to_string(), "ident");
}

#[test]
fn test_vec_gen() {
    let mut token_iter = " ident1 ident2 ".to_token_iter();

    let idents = Vec::<Ident>::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    idents.to_tokens(&mut out);
    assert_eq!(out.to_string(), "ident1 ident2");
}

#[test]
fn test_parenthesisgroup_gen() {
    let mut token_iter = " ( ident1 ident2 ) ".to_token_iter();

    let group = ParenthesisGroup::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    group.to_tokens(&mut out);
    assert_eq!(out.to_string(), "(ident1 ident2)");
}

#[test]
fn test_groupcontaining_gen() {
    let mut token_iter = " { braced } ".to_token_iter();

    let group = GroupContaining::<Ident>::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    group.to_tokens(&mut out);

    assert_eq!(out.to_string(), "{ braced }");
}

unsynn! {
    struct TupleStruct(Ident, Ident, Literal);
}

#[test]
fn test_tuple_struct() {
    let mut token_iter = r#" ident1 ident2 "literal" "#.to_token_iter();

    let tuple_struct = TupleStruct::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    tuple_struct.to_tokens(&mut out);
    assert_eq!(out.to_string(), r#"ident1 ident2 "literal""#);
}
