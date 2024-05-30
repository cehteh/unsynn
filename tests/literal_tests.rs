use unsynn::*;

#[test]
fn test_integer() {
    let mut token_iter = quote::quote! {1234}.into_iter();

    let integer = LiteralInteger::parse(&mut token_iter).unwrap();
    assert_eq!(integer.value(), 1234);
}

#[test]
fn test_character() {
    let mut token_iter = quote::quote! { 'x' }.into_iter();

    let character = LiteralCharacter::parse(&mut token_iter).unwrap();
    assert_eq!(character.value(), 'x');
}

#[test]
fn test_string() {
    let mut token_iter = quote::quote! { "this is a string literal" }.into_iter();

    let string = LiteralString::parse(&mut token_iter).unwrap();
    assert_eq!(string.value(), "\"this is a string literal\"");
}
