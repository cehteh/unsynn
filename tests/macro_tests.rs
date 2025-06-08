//! testing macros
use unsynn::*;

unsynn! {
    enum Enum {
        Two(Plus,Plus, Dot),
        One(Plus, Dot),
        TwoS{ a: Minus, b: Minus, c: Dot},
        OneS{ a: Minus, b: Dot},
        // the Expect<Dollar> shows a rust-analyzer error here, which is probably a bug in r-a
        PunctBreak(Punct, Expect<Dollar>),
    }

    type Attributes<C> = Vec<Attribute<C>>;

    struct Attribute<C: Parse> {
        _pound: Pound,
        pub outer: Option<Bang>,

        pub content: BracketGroupContaining<C>,
    }

    use std::fmt::Debug;
    // only default and where clause
    struct WithDefaultAndWhere<T = usize>
    where
        T: Debug + Send
    {
        t: T
    }
}

// Bug in 0.0.17, parsing Enum::Two consumes the Plus token
#[test]
fn test_parse_enum_consume_bug() {
    let mut i = "+.-.*$".to_token_iter();

    let parsed = i.parse::<Enum>().unwrap();
    assert!(matches!(parsed, Enum::One(..)));
    assert_tokens_eq!(parsed, "+ .");

    let parsed = i.parse::<Enum>().unwrap();
    assert!(matches!(parsed, Enum::OneS { .. }));
    assert_tokens_eq!(parsed, "- .");
}

#[test]
fn test_generics() {
    let mut i = "*$".to_token_iter();

    let parsed = i.parse::<Enum>().unwrap();
    assert!(matches!(parsed, Enum::PunctBreak(..)));
    assert_tokens_eq!(parsed, "*");

    let parsed = i.parse::<Dollar>().unwrap();
    assert_tokens_eq!(parsed, "$");
}

#[test]
fn test_keyword_default() {
    keyword! {Def = "default"};
    Def::default();
}

#[test]
fn test_quote_macro() {
    let quoted = quote! {};
    assert_tokens_eq!(quoted, "");

    let quoted = quote! {()};
    assert_tokens_eq!(quoted, "()");

    let quoted = quote! { 1 };
    assert_tokens_eq!(quoted, "1");

    let quoted = quote! { a += "2" };
    assert_tokens_eq!(quoted, r#" a += "2" "#);

    let ast = "1+2"
        .into_token_iter()
        .parse::<Cons<LiteralInteger, Plus, LiteralInteger>>()
        .unwrap();
    let quoted = quote! { let a = (#ast);};
    assert_tokens_eq!(quoted, "let a = (1+2);");
}
