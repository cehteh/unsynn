//! testing the Error trait impls
#![allow(clippy::unwrap_used)]

use unsynn::*;

static CODE: &'static str = r#"
first line
// comment
{second line}
end
"#;

unsynn! {
    struct SomeCode {
        first: Ident,
        line: Ident,
        braced: BraceGroup,
    }
}

// BUG:
#[test]
fn test_parse_fail() {
    let mut token_iter = CODE.to_token_iter();

    let err = LiteralInteger::parse_all(&mut token_iter).unwrap_err();

    let ErrorKind::UnexpectedToken { expected, at } = err.kind else {
        panic!()
    };

    assert_eq!(expected, "LiteralInteger");
    assert_eq!(Ident::parse(&mut at.into()).unwrap(), "first");
}
