use unsynn::*;

#[test]
fn test_u8_parse() {
    let mut tokens = quote::quote! {+42}.into_iter();
    let value = u8::parse(&mut tokens).unwrap();
    assert_eq!(value, 42);
}

#[test]
#[should_panic = "out of range"]
fn test_u8_parse_err() {
    let mut tokens = quote::quote! {256}.into_iter();
    let _value = u8::parse(&mut tokens).unwrap();
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_usize_parse() {
    let mut tokens = quote::quote! {123456789}.into_iter();
    let value = usize::parse(&mut tokens).unwrap();
    assert_eq!(value, 123456789);
}

#[test]
fn test_i8_parse() {
    let mut tokens = quote::quote! {-42}.into_iter();
    let value = i8::parse(&mut tokens).unwrap();
    assert_eq!(value, -42);
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_isize_parse() {
    let mut tokens = quote::quote! {-123456789}.into_iter();
    let value = isize::parse(&mut tokens).unwrap();
    assert_eq!(value, -123456789);
}

#[test]
fn test_parse_char() {
    let mut tokens = quote::quote! {'x'}.into_iter();
    let value = char::parse(&mut tokens).unwrap();
    assert_eq!(value, 'x');
}

#[test]
fn test_parse_bool() {
    let mut tokens = quote::quote! { true false }.into_iter();
    assert!(bool::parse(&mut tokens).unwrap());
    assert!(!bool::parse(&mut tokens).unwrap());
}
