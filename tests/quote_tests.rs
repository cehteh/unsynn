//! testing unsynn<->quote macro interoperability
#![cfg(feature = "quote")]

use unsynn::*;

#[test]
fn test_quote_macro() {
    let ast = "1+2"
        .into_token_iter()
        .parse::<Quoteable<Cons<LiteralInteger, Plus, LiteralInteger>>>()
        .unwrap();
    let quoted = quote! { let a = #ast;};
    assert_eq!(quoted.tokens_to_string(), "let a = 1+2;".tokens_to_string());
}
