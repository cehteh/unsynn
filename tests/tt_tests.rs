//! testing `TokenTree` parsing
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_parse_tokentree() {
    let mut token_iter = ": nopunct".to_token_iter();

    let _tokentree = TokenTree::parse(&mut token_iter).unwrap();
    assert!(TokenTree::parse(&mut token_iter).is_ok());
    assert!(TokenTree::parse(&mut token_iter).is_err());
}

#[test]
fn test_parse_group() {
    let mut token_iter = "( group ) nogroup".to_token_iter();

    let group = Group::parse(&mut token_iter).unwrap();
    assert_eq!(group.delimiter(), Delimiter::Parenthesis);
    assert!(Group::parse(&mut token_iter).is_err());
}

#[test]
fn test_parse_ident() {
    let mut token_iter = "ident 1234".to_token_iter();

    let ident = Ident::parse(&mut token_iter).unwrap();
    assert_eq!(ident.to_string(), "ident");
    assert!(Ident::parse(&mut token_iter).is_err());
}

#[test]
fn test_parse_punct() {
    let mut token_iter = ": nopunct".to_token_iter();

    let punct = Punct::parse(&mut token_iter).unwrap();
    assert_eq!(punct.as_char(), ':');
    assert!(Punct::parse(&mut token_iter).is_err());
}

#[test]
fn test_parse_literal() {
    let mut token_iter = r#""literal" noliteral"#.to_token_iter();

    let literal = Literal::parse(&mut token_iter).unwrap();
    assert_eq!(literal.to_string(), "\"literal\"");
    assert!(Literal::parse(&mut token_iter).is_err());
}

#[test]
fn test_parenthesisgroup() {
    let mut token_iter = "( content )".to_token_iter();

    let group: Group = ParenthesisGroup::parse(&mut token_iter).unwrap().into();
    assert_eq!(group.delimiter(), Delimiter::Parenthesis);
    assert_eq!(group.to_string(), "(content)");
}

#[test]
fn test_bracketgroup() {
    let mut token_iter = "[ content ]".to_token_iter();

    let group: Group = BracketGroup::parse(&mut token_iter).unwrap().into();
    assert_eq!(group.delimiter(), Delimiter::Bracket);
    assert_eq!(group.to_string(), "[content]");
}

#[test]
fn test_bracegroup() {
    let mut token_iter = "{ content }".to_token_iter();

    let group: Group = BraceGroup::parse(&mut token_iter).unwrap().into();
    assert_eq!(group.delimiter(), Delimiter::Brace);
    assert_eq!(group.to_string(), "{ content }");
}

#[test]
fn test_parse_comma() {
    let mut token_iter = ",".to_token_iter();

    let _comma = Comma::parse(&mut token_iter).unwrap();
}

#[test]
fn test_delimited() {
    let mut token_iter = " foo , bar , baz ".to_token_iter();

    let delim = Delimited::<Ident, Comma>::parse(&mut token_iter).unwrap();
    assert_eq!(delim.value.to_string(), "foo");
    assert!(delim.delimiter.is_some());

    let delim = Delimited::<Ident, Comma>::parse(&mut token_iter).unwrap();
    assert_eq!(delim.value.to_string(), "bar");
    assert!(delim.delimiter.is_some());

    let delim = Delimited::<Ident, Comma>::parse(&mut token_iter).unwrap();
    assert_eq!(delim.value.to_string(), "baz");
    assert!(delim.delimiter.is_none());
}

#[test]
fn test_delimited_undelimited() {
    let mut token_iter = " foo bar ".to_token_iter();

    let delim = Delimited::<Ident, Comma>::parse(&mut token_iter).unwrap();
    assert_eq!(delim.value.to_string(), "foo");
    assert!(delim.delimiter.is_none());

    let delim = Delimited::<Ident, Comma>::parse(&mut token_iter).unwrap();
    assert_eq!(delim.value.to_string(), "bar");
    assert!(delim.delimiter.is_none());
}

#[test]
#[should_panic = "Unexpected token: expected unsynn::fundamental::EndOfStream, found Ident"]
fn test_parse_all() {
    let mut token_iter = " foo bar ".to_token_iter();

    let _ident = Ident::parse_all(&mut token_iter).unwrap();
}
