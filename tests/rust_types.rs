//! testing rust type parsing
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_u8_parse() {
    let mut tokens = "+42".to_token_iter();
    let value = u8::parse(&mut tokens).unwrap();
    assert_eq!(value, 42);
}

#[test]
#[should_panic = "out of range"]
fn test_u8_parse_err() {
    let mut tokens = "256".to_token_iter();
    let _value = u8::parse(&mut tokens).unwrap();
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_usize_parse() {
    let mut tokens = "123456789".to_token_iter();
    let value = usize::parse(&mut tokens).unwrap();
    assert_eq!(value, 123456789);
}

#[test]
fn test_i8_parse() {
    let mut tokens = "-42".to_token_iter();
    let value = i8::parse(&mut tokens).unwrap();
    assert_eq!(value, -42);
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_isize_parse() {
    let mut tokens = "-123456789".to_token_iter();
    let value = isize::parse(&mut tokens).unwrap();
    assert_eq!(value, -123456789);
}

#[test]
fn test_parse_char() {
    let mut tokens = "'x'".to_token_iter();
    let value = char::parse(&mut tokens).unwrap();
    assert_eq!(value, 'x');
}

#[test]
fn test_parse_bool() {
    let mut tokens = " true false ".to_token_iter();
    assert!(bool::parse(&mut tokens).unwrap());
    assert!(!bool::parse(&mut tokens).unwrap());
}

#[test]
fn test_parse_string() {
    let mut tokens = r#" ident "literal" 12345 { +group } "#.to_token_iter();

    assert_eq!(String::parse(&mut tokens).unwrap(), "ident");
    assert_eq!(String::parse(&mut tokens).unwrap(), r#""literal""#);
    assert_eq!(String::parse(&mut tokens).unwrap(), "12345");
    assert_eq!(String::parse(&mut tokens).unwrap(), "{ + group }");
}
