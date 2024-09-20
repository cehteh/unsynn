use unsynn::*;

unsynn! {
    enum Enum {
        Two(Plus,Plus, Dot),
        One(Plus, Dot),
        TwoS{ a: Minus, b: Minus, c: Dot},
        OneS{ a: Minus, b: Dot},
        // the Expect<Dollar> shows a rust-analyzer error here, which is probably a bug
        PunctBreak(Punct, Expect<Dollar>),
    }
}

// Bug in 0.0.17, parsing Enum::Two consumes the Plus token
#[test]
fn test_parse_enum_consume_bug() {
    let mut i = "+.-.*$".to_token_iter();

    let parsed = i.parse::<Enum>().unwrap();
    assert!(matches!(parsed, Enum::One(..)));
    assert_eq!(parsed.tokens_to_string(), "+ .".tokens_to_string());

    let parsed = i.parse::<Enum>().unwrap();
    assert!(matches!(parsed, Enum::OneS { .. }));
    assert_eq!(parsed.tokens_to_string(), "- .".tokens_to_string());
}

#[test]
fn test_generics() {
    let mut i = "*$".to_token_iter();

    let parsed = i.parse::<Enum>().unwrap();
    assert!(matches!(parsed, Enum::PunctBreak(..)));
    assert_eq!(parsed.tokens_to_string(), "*".tokens_to_string());

    let parsed = i.parse::<Dollar>().unwrap();
    assert_eq!(parsed.tokens_to_string(), "$".tokens_to_string());
}
