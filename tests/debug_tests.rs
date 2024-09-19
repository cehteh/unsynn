#![allow(clippy::items_after_statements)]
#![cfg(any(debug_assertions, feature = "impl_debug"))]
use unsynn::*;

// Debug formats are not stable, we're stripping whitespace from strings to make the tests
// more reliable and readable. Only a few tests are included here to check it works, when it
// breaks please investigate and PR. Note: this is a hack as it joins text/tokens together.
trait StripWhitespace {
    fn strip_whitespace(self) -> String;
}

impl StripWhitespace for String {
    fn strip_whitespace(self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

impl StripWhitespace for &str {
    fn strip_whitespace(self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

#[test]
fn test_operator() {
    let mut token_iter = "->".to_token_iter();

    let example = RArrow::parse(&mut token_iter).unwrap();
    assert_eq!(
        format!("{example:?}").strip_whitespace(),
        "Operator<'->'>".strip_whitespace()
    );
}

#[test]
fn test_lazy_vec() {
    let mut token_iter = "foo bar 1 baz 2".to_token_iter();

    type Example = LazyVec<TokenTree, LiteralInteger>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(
        format!("{example:?}").strip_whitespace(),
        "LazyVec<proc_macro2::TokenTree,unsynn::literal::LiteralInteger>{
             vec:[
                 Ident{sym: foo, span:bytes(1..4)},
                 Ident{sym: bar, span:bytes(5..8)}
             ],
         terminator: LiteralInteger
         {literal:Literal{lit:1,span:bytes(9..10)},value:1}}"
            .strip_whitespace()
    );
}

#[test]
fn test_group_containing() {
    let mut token_iter = " { foo } ".to_token_iter();

    type Example = BraceGroupContaining<Ident>;

    let example = Example::parse(&mut token_iter).unwrap();
    assert_eq!(
        format!("{example:?}").strip_whitespace(),
        "BraceGroupContaining<proc_macro2::Ident>(
                 Ident{sym:foo, span:bytes(4..7)}
         )"
        .strip_whitespace()
    );
}
