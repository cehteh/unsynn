use unsynn::*;

#[test]
fn test_cons() {
    let mut token_iter = ": nopunct".to_token_iter();
    let cons = Cons::<Punct, Ident>::parse(&mut token_iter).unwrap();
    assert_eq!(cons.tokens_to_string(), ": nopunct".tokens_to_string());
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
