//! testing literal parsing
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_integer() {
    let mut token_iter = "1234".to_token_iter();

    let integer = LiteralInteger::parse(&mut token_iter).unwrap();
    assert_eq!(integer.value(), 1234);
}

#[test]
fn test_constinteger() {
    let mut token_iter = "1234".to_token_iter();
    assert!(<ConstInteger<111>>::parse(&mut token_iter).is_err());
    assert_eq!(<ConstInteger<1234>>::default().value(), 1234);
}

#[test]
fn test_character() {
    let mut token_iter = "'x'".to_token_iter();

    let character = LiteralCharacter::parse(&mut token_iter).unwrap();
    assert_eq!(character.value(), 'x');
}

#[test]
fn test_constcharacter() {
    let mut token_iter = "'f'".to_token_iter();
    assert!(<ConstCharacter<'o'>>::parse(&mut token_iter).is_err());
    assert_eq!(<ConstCharacter<'f'>>::default().value(), 'f');
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
    assert_eq!(string.value(), "\"this is a string literal\"");
    assert_eq!(string.as_str(), "this is a string literal");
    assert_eq!(string.tokens_to_string(), "\"this is a string literal\"");
}

#[test]
fn test_string_from_str() {
    let string = LiteralString::from_str("this is a string literal");
    assert_eq!(string.value(), "\"this is a string literal\"");
    assert_eq!(string.as_str(), "this is a string literal");
    // bug in v0.1.0
    assert_eq!(string.tokens_to_string(), "\"this is a string literal\"");
}

#[test]
fn test_integer_set() {
    let mut integer = LiteralInteger::new(123);
    assert_eq!(integer.value(), 123);

    integer.set(456);
    assert_eq!(integer.value(), 456);

    // Test PartialEq implementations
    assert!(integer == 456);
    assert!(!(integer == 123));
}

#[test]
fn test_character_set() {
    let mut character = LiteralCharacter::new('a');
    assert_eq!(character.value(), 'a');

    character.set('b');
    assert_eq!(character.value(), 'b');

    // Test PartialEq implementations
    assert!(character == 'b');
    assert!(!(character == 'a'));
}

#[test]
fn test_string_set() {
    let mut string = LiteralString::from_str("hello");
    assert_eq!(string.value(), "\"hello\"");

    string.set("\"world\"".to_string());
    assert_eq!(string.value(), "\"world\"");

    // Test PartialEq implementations
    assert!(string == "world");
    assert!(!(string == "hello"));
}

#[test]
fn test_integer_partial_eq() {
    let integer = LiteralInteger::new(42);
    assert!(integer == 42u128);
    assert!(!(integer == 43u128));
}

#[test]
fn test_character_partial_eq() {
    let character = LiteralCharacter::new('a');
    assert!(character == 'a');
    assert!(!(character == 'b'));
}

#[test]
fn test_string_partial_eq() {
    let string = LiteralString::from_str("test");
    assert!(string == "test");
    assert!(!(string == "other"));
}

#[test]
fn test_integer_set_value() {
    let mut integer = LiteralInteger::new(100);
    integer.set(200);
    assert_eq!(integer.value(), 200);
}

#[test]
fn test_character_set_value() {
    let mut character = LiteralCharacter::new('x');
    character.set('y');
    assert_eq!(character.value(), 'y');
}

#[test]
fn test_string_set_value() {
    let mut string = LiteralString::from_str("old");
    string.set("\"new\"".to_string());
    assert_eq!(string.value(), "\"new\"");
    assert_eq!(string.as_str(), "new");
}
