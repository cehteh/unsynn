use unsynn::*;

#[test]
#[should_panic = "Unexpected token: expected proc_macro2::Ident, found Group"]
fn test_error_unexpected_token() {
    let mut token_iter = quote::quote! {( group )}.into_iter();

    let _ident = Ident::parse(&mut token_iter).unwrap();
}

#[test]
#[should_panic = "Unexpected end of input:"]
fn test_error_unexpected_end() {
    let mut token_iter = quote::quote! {}.into_iter();

    let _ident = Ident::parse(&mut token_iter).unwrap();
}

keyword! {Frob = "frob"}

#[test]
#[should_panic = "keyword \"frob\" expected, got \"nofrob\""]
fn test_error_keyword() {
    let mut token_iter = quote::quote! {nofrob}.into_iter();

    let _ident = Frob::parse(&mut token_iter).unwrap();
}
