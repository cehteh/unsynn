//! testing Group parsing
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_group_contains() {
    let mut token_iter = " ( ident ) ".to_token_iter();

    let group_containing = ParenthesisGroupContaining::<Ident>::parse(&mut token_iter).unwrap();

    assert_eq!(group_containing.delimiter(), Delimiter::Parenthesis);
    assert_eq!(group_containing.content.to_string(), "ident");
}

#[test]
fn test_group_contains_empty() {
    let mut token_iter = " {} ".to_token_iter();

    let group_containing = BraceGroupContaining::<Nothing>::parse(&mut token_iter).unwrap();

    assert_eq!(group_containing.delimiter(), Delimiter::Brace);
}

#[test]
#[should_panic = "Unexpected token: expected unsynn::fundamental::EndOfStream"]
fn test_group_contains_leftover_tokens() {
    let mut token_iter = " { leftover } ".to_token_iter();

    let group_containing = BraceGroupContaining::<Nothing>::parse(&mut token_iter).unwrap();

    assert_eq!(group_containing.delimiter(), Delimiter::Brace);
}
