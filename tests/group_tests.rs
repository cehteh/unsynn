use unsynn::*;

#[test]
fn test_group_contains() {
    let mut token_iter = quote::quote! { ( ident ) }.into_iter();

    let group_containing =
        GroupContaining::<ParenthesisGroup, Ident>::parse(&mut token_iter).unwrap();

    assert_eq!(group_containing.delimiter, Delimiter::Parenthesis);
    assert_eq!(group_containing.content.to_string(), "ident");
}

#[test]
fn test_group_contains_empty() {
    let mut token_iter = quote::quote! { {} }.into_iter();

    let group_containing = GroupContaining::<BraceGroup, Nothing>::parse(&mut token_iter).unwrap();

    assert_eq!(group_containing.delimiter, Delimiter::Brace);
}
