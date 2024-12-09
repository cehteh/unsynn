//! testing combinators
#![allow(clippy::unwrap_used)]
use unsynn::*;

#[test]
fn test_cons() {
    let mut token_iter = ": nopunct".to_token_iter();
    let cons = Cons::<Punct, Ident>::parse(&mut token_iter).unwrap();
    assert_eq!(cons.tokens_to_string(), ": nopunct".tokens_to_string());
}

#[test]
fn test_4cons() {
    let mut token_iter = ": nopunct 'c' 123".to_token_iter();
    let cons =
        Cons::<Punct, Ident, LiteralCharacter, LiteralInteger>::parse(&mut token_iter).unwrap();
    assert_eq!(
        cons.tokens_to_string(),
        ": nopunct 'c' 123".tokens_to_string()
    );
}

#[test]
fn test_except() {
    let mut token_iter = ": nopunct".to_token_iter();
    assert!(Except::<Ident>::parse(&mut token_iter).is_ok());
    assert!(Except::<Punct>::parse(&mut token_iter).is_err());
}

#[test]
fn test_either() {
    let mut token_iter = ": nopunct".to_token_iter();

    let Either::First(_) = Either::<Punct, Ident>::parse(&mut token_iter).unwrap() else {
        unreachable!();
    };
    let Either::Second(_) = Either::<Punct, Ident>::parse(&mut token_iter).unwrap() else {
        unreachable!();
    };
}

// test that the error which made the most progress is returned from either
#[test]
fn test_either_error_progress() {
    let mut token_iter = ": : ()".to_token_iter();
    let error1 = Either::<Cons<Punct, Ident>, Cons<Punct, Punct, Punct>>::parse(&mut token_iter)
        .unwrap_err();
    let error2 = Either::<Cons<Punct, Punct, Punct>, Cons<Punct, Ident>>::parse(&mut token_iter)
        .unwrap_err();
    assert_eq!(error1.to_string(), error2.to_string());
}

#[test]
fn test_either_to_tokens() {
    let mut tokens = TokenStream::new();

    let either: Either<Punct, Ident> = Either::First(Punct::new(':', Spacing::Alone));
    either.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), ":");

    tokens = TokenStream::new();
    let either: Either<Punct, Ident> = Either::Second(Ident::new("test", Span::call_site()));
    either.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), "test");
}
