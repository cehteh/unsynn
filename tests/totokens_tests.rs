use unsynn::*;

#[test]
fn test_gen() {
    let mut token_iter = quote::quote! { ident }.into_iter();

    let ident = Ident::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    ident.to_tokens(&mut out);
    assert_eq!(out.to_string(), "ident");
}

#[test]
fn test_vec_gen() {
    let mut token_iter = quote::quote! { ident1 ident2 }.into_iter();

    let idents = Vec::<Ident>::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    idents.to_tokens(&mut out);
    assert_eq!(out.to_string(), "ident1 ident2");
}

#[test]
fn test_parenthesisgroup_gen() {
    let mut token_iter = quote::quote! { ( ident1 ident2 ) }.into_iter();

    let group = ParenthesisGroup::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    group.to_tokens(&mut out);
    assert_eq!(out.to_string(), "(ident1 ident2)");
}

#[test]
fn test_groupcontaining_gen() {
    let mut token_iter = quote::quote! { { braced } }.into_iter();

    let group = GroupContaining::<BraceGroup, Ident>::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    group.to_tokens(&mut out);

    assert_eq!(out.to_string(), "{ braced }");
}

unsynn! {
    struct TupleStruct(Ident, Ident, Literal);
}

#[test]
fn test_tuple_struct() {
    let mut token_iter = quote::quote! { ident1 ident2 "literal" }.into_iter();

    let tuple_struct = TupleStruct::parse(&mut token_iter).unwrap();

    let mut out = TokenStream::new();
    tuple_struct.to_tokens(&mut out);
    assert_eq!(out.to_string(), "ident1 ident2 \"literal\"");
}
