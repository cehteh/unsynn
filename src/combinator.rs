//! A unique feature of unsynn is that one can define a parser as a composition of other
//! parsers on the fly without the need to define custom structures. This is done by using the
//! `Cons` and `Either` types. The [`Cons`] type is used to define a parser that is a
//! conjunction of two other parsers, while the [`Either`] type is used to define a parser
//! that is a disjunction of two other parsers.

use crate::{Parse, Parser, Result, ToTokens, TokenIter, TokenStream, TokenTree};

/// Conjunctive `A` followed by `B`
#[derive(Clone)]
pub struct Cons<A: Parse, B: Parse> {
    /// The first value
    pub first: A,
    /// The second value
    pub second: B,
}

impl<A: Parse, B: Parse> Parser for Cons<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self {
            first: A::parser(tokens)?,
            second: B::parser(tokens)?,
        })
    }
}

impl<A: Parse + ToTokens, B: Parse + ToTokens> ToTokens for Cons<A, B> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.first.to_tokens(tokens);
        self.second.to_tokens(tokens);
    }
}

impl<A: Parse, B: Parse> From<Cons<A, B>> for (A, B) {
    fn from(cons: Cons<A, B>) -> Self {
        (cons.first, cons.second)
    }
}

#[cfg(feature = "impl_debug")]
impl<A: Parse + std::fmt::Debug, B: Parse + std::fmt::Debug> std::fmt::Debug for Cons<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(&format!(
            "Cons<{}, {}>",
            std::any::type_name::<A>(),
            std::any::type_name::<B>()
        ))
        .field("first", &self.first)
        .field("second", &self.second)
        .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<A: Parse + std::fmt::Display, B: Parse + std::fmt::Display> std::fmt::Display for Cons<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.first, self.second)
    }
}

/// Disjunctive `A` or `B` tried in that order.
#[derive(Clone)]
pub enum Either<A: Parse, B: Parse> {
    /// The first alternative
    First(A),
    /// The second alternative
    Second(B),
}

impl<A: Parse, B: Parse> Either<A, B> {
    /// Deconstructs an `Either` and produces a common result type, independent of which
    /// alternative was present.
    pub fn fold<R, FF: FnOnce(A) -> R, SF: FnOnce(B) -> R>(self, first_fn: FF, second_fn: SF) -> R {
        match self {
            Either::First(a) => first_fn(a),
            Either::Second(b) => second_fn(b),
        }
    }
}

impl<A: Parse, B: Parse> Parser for Either<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        if let Ok(first) = A::parse(tokens) {
            Ok(Either::First(first))
        } else {
            Ok(Either::Second(B::parser(tokens)?))
        }
    }
}

impl<A: Parse + ToTokens, B: Parse + ToTokens> ToTokens for Either<A, B> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Either::First(a) => a.to_tokens(tokens),
            Either::Second(b) => b.to_tokens(tokens),
        }
    }
}

#[cfg(feature = "impl_debug")]
impl<A: Parse + std::fmt::Debug, B: Parse + std::fmt::Debug> std::fmt::Debug for Either<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Either::First(a) => f
                .debug_tuple(&format!(
                    "Either<{}, {}>::First",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>()
                ))
                .field(a)
                .finish(),
            Either::Second(b) => f.debug_tuple("Either::Second").field(b).finish(),
        }
    }
}

#[cfg(feature = "impl_display")]
impl<A: Parse + std::fmt::Display, B: Parse + std::fmt::Display> std::fmt::Display
    for Either<A, B>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Either::First(a) => write!(f, "{a}"),
            Either::Second(b) => write!(f, "{b}"),
        }
    }
}

impl<A: Parse + Into<TokenTree>, B: Parse + Into<TokenTree>> From<Either<A, B>> for TokenTree {
    fn from(either: Either<A, B>) -> Self {
        match either {
            Either::First(a) => a.into(),
            Either::Second(b) => b.into(),
        }
    }
}

#[test]
fn test_either_into_tt() {
    use crate::LiteralInteger;
    let either = Either::<LiteralInteger, TokenTree>::First(LiteralInteger::new(42));
    let _tt: TokenTree = either.into();

    let either = Either::<TokenTree, LiteralInteger>::Second(LiteralInteger::new(43));
    let _tt: TokenTree = either.into();
}
