use unsynn::*;

#[test]
fn test_integer() {
    let mut token_iter = "1234".to_token_iter();

    let integer = LiteralInteger::parse(&mut token_iter).unwrap();
    assert_eq!(integer.value(), 1234);
}

#[test]
fn test_character() {
    let mut token_iter = "'x'".to_token_iter();

    let character = LiteralCharacter::parse(&mut token_iter).unwrap();
    assert_eq!(character.value(), 'x');
}

#[test]
fn test_string() {
    let mut token_iter = r#" "this is a string literal" "#.to_token_iter();

    let string = LiteralString::parse(&mut token_iter).unwrap();
    assert_eq!(string.value(), "\"this is a string literal\"");
}

#[test]
fn test_string_new() {
    let string = LiteralString::new("\"this is a string literal\"".to_string());
    assert_eq!(string.value(), "\"this is a string literal\"");
}

#[test]
#[should_panic = "assertion failed: value.starts_with('\"') && value.ends_with('\"')"]
fn test_string_new_err() {
    let string = LiteralString::new("this is a string literal".to_string());
    assert_eq!(string.value(), "\"this is a string literal\"");
}

#[test]
fn test_string_as_str() {
    let mut token_iter = r#" "this is a string literal" "#.to_token_iter();

    let string = LiteralString::parse(&mut token_iter).unwrap();
    assert_eq!(string.as_str(), "this is a string literal");
}

#[test]
fn test_string_from_str() {
    let string = LiteralString::from_str("this is a string literal");
    assert_eq!(string.value(), "\"this is a string literal\"");
}
