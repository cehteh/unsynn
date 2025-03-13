//! testing the Error trait impls
#![allow(clippy::unwrap_used)]

use unsynn::*;

#[test]
#[should_panic = "Unexpected token: expected proc_macro2::Ident"]
fn test_error_unexpected_token() {
    let mut token_iter = "( group )".to_token_iter();

    let _ident = Ident::parse(&mut token_iter).unwrap();
}

#[test]
fn test_error_set_pos() {
    let mut err = Error::no_error();

    assert_eq!(err.pos(), 0);
    err.set_pos(42);
    assert_eq!(err.pos(), 42);
}

#[test]
#[should_panic = "found TokenStream [] at None"]
fn test_error_unexpected_end() {
    let mut token_iter = "".to_token_iter();

    let _ident = Ident::parse(&mut token_iter).unwrap();
}

keyword! {Frob = "frob"}

#[test]
#[should_panic = r#"keyword "frob" expected, got "nofrob""#]
fn test_error_keyword() {
    let mut token_iter = "nofrob".to_token_iter();

    let _ident = Frob::parse(&mut token_iter).unwrap();
}

#[test]
fn test_token_count_impls() {
    let mut tokens = "a b c".to_token_iter();

    // Advance the counter by consuming a token
    tokens.next();
    let expected_count = tokens.counter();
    assert_eq!(expected_count, 1); // After consuming one token

    // Test that all implementations return different values than a fixed '1'
    tokens.next(); // advance again to make sure we're not at position 1
    let current_count = tokens.counter();
    assert!(current_count > 1); // Verify we're past position 1

    // Test TokenCount for usize (direct position)
    assert_eq!(current_count.token_count(), current_count);
    assert_ne!(current_count.token_count(), 1);

    // Test TokenCount for &TokenIter
    assert_eq!((&tokens).token_count(), current_count);
    assert_ne!((&tokens).token_count(), 1);

    // Test TokenCount for &&TokenIter
    assert_eq!((&(&tokens)).token_count(), current_count);
    assert_ne!((&(&tokens)).token_count(), 1);

    // Test TokenCount for &mut TokenIter
    assert_eq!((&mut tokens).token_count(), current_count);
    assert_ne!((&mut tokens).token_count(), 1);

    // Test TokenCount for &&mut TokenIter
    assert_eq!((&(&mut tokens)).token_count(), current_count);
    assert_ne!((&(&mut tokens)).token_count(), 1);
}

#[test]
fn test_error_upgrade() {
    let mut err = Error::no_error();
    let mut tokens = "a b c".to_token_iter();

    // Create first error at initial position
    let result1: Result<Punct> = Error::unexpected_token(&tokens);
    let err1_str = result1.as_ref().unwrap_err().to_string();
    let _ = err.upgrade(result1).expect_err("should be an error");
    assert_eq!(err.to_string(), err1_str);

    // Advance counter by consuming a token
    tokens.next();

    // Second error at later position should replace first error
    let result2: Result<Punct> = Error::unexpected_token(&tokens);
    let err2_str = result2.as_ref().unwrap_err().to_string();
    let _ = err
        .upgrade(result2.clone())
        .expect_err("should be an error");
    assert_eq!(err.to_string(), err2_str);

    // Earlier position error should not replace later error
    let _ = err
        .upgrade::<Punct>(Error::unexpected_token(&tokens))
        .expect_err("should be an error");
    assert_eq!(err.to_string(), err2_str);
}
