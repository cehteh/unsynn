use unsynn::*;

#[test]
fn test_groups_contains() {
    let mut token_iter = quote::quote! { ( ident ) }.into_iter();

    let group_containing =
        GroupContaining::<ParenthesisGroup, Ident>::parse(&mut token_iter).unwrap();

    assert_eq!(
        group_containing.group.as_group().delimiter(),
        Delimiter::Parenthesis
    );
    assert_eq!(group_containing.content.to_string(), "ident");
}
