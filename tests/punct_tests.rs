use unsynn::*;

#[test]
fn test_onepunct() {
    let mut token_iter = quote::quote! {; nopunct}.into_iter();

    let semi = Semicolon::parse(&mut token_iter).unwrap();
    assert_eq!(semi.to_string(), ";");
}

#[test]
fn test_twopunct() {
    let mut token_iter = quote::quote! {-> nopunct}.into_iter();

    let arrow = RArrow::parse(&mut token_iter).unwrap();
    assert_eq!(arrow.to_string(), "->");
}
