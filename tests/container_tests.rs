//! testing containers
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_option() {
    let mut token_iter = ": nopunct".to_token_iter();

    let maybe_punct = Option::<Punct>::parse(&mut token_iter).unwrap();
    assert!(maybe_punct.is_some());

    let maybe_punct2 = dbg!(Option::<Punct>::parse(&mut token_iter).unwrap());
    assert!(maybe_punct2.is_none());
}

#[test]
fn test_vec() {
    let mut token_iter = ":::::::nopunct".to_token_iter();

    let args = Vec::<Punct>::parse(&mut token_iter).unwrap();
    assert_eq!(args.len(), 7);
    let noargs = Vec::<Punct>::parse(&mut token_iter).unwrap();
    assert!(noargs.is_empty());
    let idents = Vec::<Ident>::parse(&mut token_iter).unwrap();
    assert_eq!(idents.len(), 1);
}

#[test]
fn test_rc_refcell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut token_iter = " test ".to_token_iter();

    let ident = Rc::<RefCell<Ident>>::parse(&mut token_iter).unwrap();
    assert_eq!(ident.borrow().to_string(), "test");
}

#[test]
fn test_vec_delimited() {
    // note that the comma is optional
    let mut token_iter = " foo , bar baz ".to_token_iter();

    let vec = Vec::<CommaDelimited<Ident>>::parse(&mut token_iter).unwrap();
    assert_eq!(vec[0].value.to_string(), "foo");
    assert!(vec[0].delimiter.is_some());
    assert_eq!(vec[1].value.to_string(), "bar");
    assert!(vec[1].delimiter.is_none()); // <- attention!
    assert_eq!(vec[2].value.to_string(), "baz");
    assert!(vec[2].delimiter.is_none());
}

#[test]
fn test_delimitedvec() {
    // note that the missing comma stops parsing
    let mut token_iter = " foo , bar baz ".to_token_iter();

    let vec = CommaDelimitedVec::<Ident>::parse(&mut token_iter)
        .unwrap()
        .0;
    assert_eq!(vec[0].value.to_string(), "foo");
    assert!(vec[0].delimiter.is_some());
    assert_eq!(vec[1].value.to_string(), "bar");
    assert!(vec[1].delimiter.is_none());
    assert_eq!(vec.len(), 2);
}

#[test]
fn test_nothingdelimitedvec() {
    let mut token_iter = " foo bar baz ".to_token_iter();

    let vec = DelimitedVec::<Ident, Nothing>::parse(&mut token_iter)
        .unwrap()
        .0;
    assert_eq!(vec[0].value.to_string(), "foo");
    assert!(vec[0].delimiter.is_some());
    assert_eq!(vec[1].value.to_string(), "bar");
    assert!(vec[1].delimiter.is_some());
    assert_eq!(vec[2].value.to_string(), "baz");
    assert!(vec[2].delimiter.is_some());
}

#[test]
fn test_repeats() {
    let mut token_iter = " foo bar baz ".to_token_iter();

    let vec = Exactly::<2, Ident>::parse(&mut token_iter).unwrap().0;
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0].value.to_string(), "foo");
    assert_eq!(vec[1].value.to_string(), "bar");
}

#[test]
fn test_vec_ranged_repeats() {
    let mut token_iter = "a b c d e".to_token_iter();

    // Test minimum boundary - will parse up to max elements when available
    let result = Vec::<Ident>::parse_repeats(&mut token_iter, 3, 5).unwrap();
    assert_eq!(result.len(), 5);

    // Test error case for too few elements
    let mut token_iter = "a b".to_token_iter();
    assert!(Vec::<Ident>::parse_repeats(&mut token_iter, 3, 5).is_err());
}

#[test]
fn test_lazy_vec_ranged_repeats() {
    let mut token_iter = "a b c;".to_token_iter();

    // Test valid case within min/max bounds
    let result = LazyVec::<Ident, Semicolon>::parse_repeats(&mut token_iter, 2, 5).unwrap();
    assert_eq!(result.vec.len(), 3);

    // Test error case for too few elements
    let mut token_iter = "a;".to_token_iter();
    assert!(LazyVec::<Ident, Semicolon>::parse_repeats(&mut token_iter, 2, 5).is_err());
}

#[test]
fn test_delimited_vec_ranged_repeats() {
    let mut token_iter = "a, b, c".to_token_iter();

    // Test valid case
    let result = DelimitedVec::<Ident, Comma>::parse_repeats(&mut token_iter, 2, 4).unwrap();
    assert_eq!(result.0.len(), 3);

    // Test error case for too few elements
    let mut token_iter = "a,".to_token_iter();
    assert!(DelimitedVec::<Ident, Comma>::parse_repeats(&mut token_iter, 2, 4).is_err());
}

#[test]
fn test_conversions() {
    // Test Vec conversion from DelimitedVec
    let mut token_iter = "a, b, c".to_token_iter();
    let delimited = DelimitedVec::<Ident, Comma>::parse(&mut token_iter).unwrap();
    let vec: Vec<Ident> = delimited.into();
    assert_eq!(vec.len(), 3);

    // Test Vec conversion from Repeats
    let mut token_iter = "a b c".to_token_iter();
    let repeats = Exactly::<3, Ident>::parse(&mut token_iter).unwrap();
    let vec: Vec<Ident> = repeats.into();
    assert_eq!(vec.len(), 3);
}

#[test]
fn test_into_iter() {
    // Test LazyVec IntoIterator
    let mut token_iter = "a b c;".to_token_iter();
    let lazy_vec = LazyVec::<Ident, Semicolon>::parse(&mut token_iter).unwrap();
    let collected: Vec<_> = lazy_vec.into_iter().collect();
    assert_eq!(collected.len(), 3);

    // Test DelimitedVec IntoIterator
    let mut token_iter = "a, b, c".to_token_iter();
    let delimited_vec = DelimitedVec::<Ident, Comma>::parse(&mut token_iter).unwrap();
    let collected: Vec<_> = delimited_vec.into_iter().collect();
    assert_eq!(collected.len(), 3);

    // Test Repeats IntoIterator
    let mut token_iter = "a b c".to_token_iter();
    let repeats = Exactly::<3, Ident>::parse(&mut token_iter).unwrap();
    let collected: Vec<_> = repeats.into_iter().collect();
    assert_eq!(collected.len(), 3);
}

#[test]
fn test_to_tokens() {
    // Test Box to_tokens
    let mut token_iter = "test".to_token_iter();
    let boxed = Box::<Ident>::parse(&mut token_iter).unwrap();
    assert_tokens_eq!(boxed, "test");

    // Test Rc to_tokens
    let mut token_iter = "test".to_token_iter();
    let rc = std::rc::Rc::<Ident>::parse(&mut token_iter).unwrap();
    assert_tokens_eq!(rc, "test");

    // Test RefCell to_tokens
    let mut token_iter = "test".to_token_iter();
    let refcell = std::cell::RefCell::<Ident>::parse(&mut token_iter).unwrap();
    assert_tokens_eq!(refcell, "test");
}

#[test]
fn test_repeats_to_tokens() {
    // Test Repeats to_tokens implementation
    let mut token_iter = " a b c ".to_token_iter();
    let repeats = Exactly::<3, Ident>::parse(&mut token_iter).unwrap();

    // Verify that elements are properly converted to tokens
    let mut tokens = TokenStream::new();
    repeats.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "a b c");

    assert_tokens_eq!(repeats, str "a b c");
}

#[test]
fn test_lazy_vec_to_tokens() {
    // Test LazyVec to_tokens implementation
    let mut token_iter = " a b c ; ".to_token_iter();
    let lazy_vec = LazyVec::<Ident, Semicolon>::parse(&mut token_iter).unwrap();

    // Verify that elements and terminator are properly converted to tokens
    let mut tokens = TokenStream::new();
    lazy_vec.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "a b c ;");

    assert_tokens_eq!(lazy_vec, str "a b c ;");
}
