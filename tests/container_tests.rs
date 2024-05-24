use unsynn::*;

#[test]
fn test_option() {
    let mut token_iter = quote::quote! {: nopunct}.into_iter();

    let maybe_punct = Option::<Punct>::parse(&mut token_iter).unwrap();
    assert!(maybe_punct.is_some());

    let maybe_punct2 = dbg!(Option::<Punct>::parse(&mut token_iter).unwrap());
    assert!(maybe_punct2.is_none());
}

#[test]
fn test_vec() {
    let mut token_iter = quote::quote! {:::::::nopunct}.into_iter();

    let args = Vec::<Punct>::parse(&mut token_iter).unwrap();
    assert_eq!(args.len(), 7);
    let noargs = Vec::<Punct>::parse(&mut token_iter).unwrap();
    assert!(noargs.is_empty());
}

#[test]
fn test_rc_refcell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut token_iter = quote::quote! { test }.into_iter();

    let ident = Rc::<RefCell<Ident>>::parse(&mut token_iter).unwrap();
    assert_eq!(ident.borrow().to_string(), "test");
}

#[test]
fn test_vec_delimited() {
    // note that the comma is optional
    let mut token_iter = quote::quote! { foo , bar baz }.into_iter();

    let vec = Vec::<CommaDelimited<Ident>>::parse(&mut token_iter).unwrap();
    assert_eq!(vec[0].0.to_string(), "foo");
    assert!(vec[0].1.is_some());
    assert_eq!(vec[1].0.to_string(), "bar");
    assert!(vec[1].1.is_none()); // <- attention!
    assert_eq!(vec[2].0.to_string(), "baz");
    assert!(vec[2].1.is_none());
}

#[test]
fn test_delimitedvec() {
    // note that the missing comma stops parsing
    let mut token_iter = quote::quote! { foo , bar baz }.into_iter();

    let vec = CommaDelimitedVec::<Ident>::parse(&mut token_iter)
        .unwrap()
        .0;
    assert_eq!(vec[0].0.to_string(), "foo");
    assert!(vec[0].1.is_some());
    assert_eq!(vec[1].0.to_string(), "bar");
    assert!(vec[1].1.is_none());
    assert_eq!(vec.len(), 2);
}

#[test]
fn test_nothingdelimitedvec() {
    let mut token_iter = quote::quote! { foo bar baz }.into_iter();

    let vec = DelimitedVec::<Ident, Nothing>::parse(&mut token_iter)
        .unwrap()
        .0;
    assert_eq!(vec[0].0.to_string(), "foo");
    assert!(vec[0].1.is_some());
    assert_eq!(vec[1].0.to_string(), "bar");
    assert!(vec[1].1.is_some());
    assert_eq!(vec[2].0.to_string(), "baz");
    assert!(vec[2].1.is_some());
}

#[test]
fn test_repeats() {
    let mut token_iter = quote::quote! { foo bar baz }.into_iter();

    let vec = Exactly::<2, Ident>::parse(&mut token_iter).unwrap().0;
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0].0.to_string(), "foo");
    assert_eq!(vec[1].0.to_string(), "bar");
}
