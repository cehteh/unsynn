//! testing the Error trait impls
#![allow(clippy::unwrap_used)]

use unsynn::*;

static CODE: &str = r"
first line
// comment
    {fourth line}
end
";

unsynn! {
    struct SomeCode {
        first: Ident,
        line: Ident,
        braced: BraceGroup,
    }
}

#[test]
fn test_parse_fail() {
    let mut token_iter = CODE.to_token_iter();

    let err = LiteralInteger::parse_all(&mut token_iter).unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedToken));

    assert_eq!(err.expected_original_type_name(), "proc_macro2::Literal");
    assert_eq!(err.expected_type_name(), "unsynn::literal::LiteralInteger");
    assert_eq!(err.failed_at().tokens_to_string(), "first");
    assert_eq!(
        Ident::parse(&mut err.tokens_after())
            .unwrap()
            .tokens_to_string(),
        "line"
    );
}

#[test]
fn test_parse_fail_check_span() {
    let mut token_iter = CODE.to_token_iter();

    let err = <Cons<Ident, Ident, LiteralString>>::parse(&mut token_iter).unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedToken));

    assert_eq!(err.expected_original_type_name(), "proc_macro2::Literal");
    assert_eq!(err.expected_type_name(), "unsynn::literal::LiteralString");
    assert_eq!(
        err.failed_at().tokens_to_string(),
        "{fourth line}".tokens_to_string()
    );
    assert_eq!(err.failed_at().map(|t| t.span().start().line), Some(4));
    assert_eq!(err.failed_at().map(|t| t.span().start().column), Some(4));

    assert_eq!(err.tokens_after().counter(), 4);
}
