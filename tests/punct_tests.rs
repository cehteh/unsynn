//! testing Punct parsing
#![allow(clippy::unwrap_used)]
#![allow(unused_variables)]
use unsynn::*;

#[test]
fn test_onepunct() {
    let mut token_iter = "; nopunct".to_token_iter();

    let semi = Semicolon::parse(&mut token_iter).unwrap();
    assert_eq!(semi.tokens_to_string(), ";");
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

operator! {
    Fancy = "<~~>";
}

#[test]
fn test_fancy() {
    let mut token_iter = "<~~>".to_token_iter();

    let fancy = Fancy::parse(&mut token_iter).unwrap();
    assert_eq!(fancy.tokens_to_string(), "<~~>");
}

#[test]
fn test_joint_text() {
    assert_eq!("<text>".tokens_to_string(), "< text >");
}

#[test]
fn test_punct_any_tokens() {
    let mut tokens = TokenStream::new();
    PunctAny::<'+'>.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "+");
}

#[test]
fn test_punct_joint_parser() {
    let mut tokens = "+=".to_token_iter();
    let punct = PunctJoint::<'+'>::parse(&mut tokens).unwrap();
    assert_eq!(punct.as_char(), '+');

    // Test error case
    let mut tokens = "+ =".to_token_iter(); // Space between + and = makes it Alone
    assert!(PunctJoint::<'+'>::parse(&mut tokens).is_err());
}

#[test]
fn test_punct_joint_tokens() {
    let mut tokens = TokenStream::new();
    PunctJoint::<'+'>.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "+");
}

#[test]
fn test_punct_alone_parser() {
    let mut tokens = "+ ".to_token_iter();
    let punct = PunctAlone::<'+'>::parse(&mut tokens).unwrap();
    assert_eq!(punct.as_char(), '+');

    // Test error case
    let mut tokens = "+=".to_token_iter(); // Joint spacing should fail
    assert!(PunctAlone::<'+'>::parse(&mut tokens).is_err());
}

#[test]
fn test_punct_alone_tokens() {
    let mut tokens = TokenStream::new();
    PunctAlone::<'+'>.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "+");
}

#[test]
fn test_operator_parser() {
    // Test single character
    let mut tokens = "+ ".to_token_iter();
    let op = Operator::<'+'>::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "+");

    // Test two characters
    let mut tokens = "+= ".to_token_iter();
    let op = Operator::<'+', '='>::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "+=");

    // Test three characters
    let mut tokens = "... ".to_token_iter();
    let op = Operator::<'.', '.', '.'>::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "...");

    // Test four characters
    let mut tokens = "<=>= ".to_token_iter();
    let op = Operator::<'<', '=', '>', '='>::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "<=>=");
}

#[test]
fn test_operator_tokens() {
    // Test that operators maintain proper spacing in token stream
    let op = Operator::<'+', '='>;
    let mut tokens = TokenStream::new();
    op.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "+=");

    let op = Operator::<'.', '.', '.'>;
    let mut tokens = TokenStream::new();
    op.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "...");
}

#[test]
fn test_operator_custom() {
    operator! {
        CustomOp = "<?>"
    }

    let mut tokens = "<?> ".to_token_iter();
    let op = CustomOp::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "<?>");
}

#[test]
fn test_predefined_operators() {
    // Test some of the predefined operators
    let mut tokens = "+= ".to_token_iter();
    let op = PlusEq::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "+=");

    let mut tokens = "-> ".to_token_iter();
    let op = RArrow::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "->");

    let mut tokens = "... ".to_token_iter();
    let op = Ellipsis::parse(&mut tokens).unwrap();
    assert_eq!(op.tokens_to_string(), "...");
}

#[test]
fn test_lifetime_tick() {
    let mut tokens = "'a".to_token_iter();
    let tick = LifetimeTick::parse(&mut tokens).unwrap();
    assert_eq!(tick.as_char(), '\'');
}
