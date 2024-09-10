#![allow(clippy::unwrap_used)]
#![allow(unused_variables)]
use unsynn::*;

#[test]
fn test_onepunct() {
    let mut token_iter = "; nopunct".to_token_iter();

    let semi = Semicolon::parse(&mut token_iter).unwrap();
    assert_eq!(semi.as_char(), ';');
}

#[test]
fn test_twopunct() {
    let mut token_iter = "-> nopunct".to_token_iter();

    let arrow = RArrow::parse(&mut token_iter).unwrap();
    assert_eq!(arrow.tokens_to_string(), "->");
}

#[test]
fn test_threepunct() {
    let mut token_iter = "... nopunct".to_token_iter();

    let ellipsis = Ellipsis::parse(&mut token_iter).unwrap();
    assert_eq!(ellipsis.tokens_to_string(), "...");
}
