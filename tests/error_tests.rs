use unsynn::*;

#[test]
#[should_panic = "Unexpected token: expected proc_macro2::Ident, found Group"]
fn test_error_unexpected_token() {
    let mut token_iter = "( group )".to_token_iter();

    let _ident = Ident::parse(&mut token_iter).unwrap();
}

#[test]
#[should_panic = "Unexpected end of input:"]
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
