//! A unique feature of unsynn is that one can define a parser as a composition of other
//! parsers on the fly without the need to define custom structures. This is done by using
//! the `Cons` and `Either` types. The `Cons` type is used to define a parser that is a
//! conjunction of two other parsers, while the `Either` type is used to define a parser
//! that is a disjunction of two other parsers.

use crate::{Parse, Parser, Result, ToTokens, TokenIter, TokenStream};

/// Conjunctive `A` followed by `B`
pub struct Cons<A: Parse, B: Parse>(A, B);

impl<A: Parse, B: Parse> Parser for Cons<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(A::parser(tokens)?, B::parser(tokens)?))
    }
}

impl<A: Parse, B: Parse> ToTokens for Cons<A, B> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
        self.1.to_tokens(tokens);
    }
}

/// Disjunctive `A` or `B` tried in that order.
pub enum Either<A: Parse, B: Parse> {
    /// The first alternative
    First(A),
    /// The second alternative
    Second(B),
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

impl<A: Parse, B: Parse> ToTokens for Either<A, B> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Either::First(a) => a.to_tokens(tokens),
            Either::Second(b) => b.to_tokens(tokens),
        }
    }
}
