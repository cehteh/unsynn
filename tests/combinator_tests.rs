use unsynn::*;

#[test]
fn test_cons() {
    let mut token_iter = quote::quote! {: nopunct}.into_iter();
    let _cons = Cons::<Punct, Ident>::parse(&mut token_iter).unwrap();
}

#[test]
fn test_except() {
    let mut token_iter = quote::quote! {: nopunct}.into_iter();
    let _except = Except::<Ident>::parse(&mut token_iter).unwrap();
    assert!(Except::<Punct>::parse(&mut token_iter).is_err());
}

#[test]
fn test_either() {
    let mut token_iter = quote::quote! {: nopunct}.into_iter();

    let Either::First(_) = Either::<Punct, Ident>::parse(&mut token_iter).unwrap() else {
        unreachable!();
    };
    let Either::Second(_) = Either::<Punct, Ident>::parse(&mut token_iter).unwrap() else {
        unreachable!();
    };
}
