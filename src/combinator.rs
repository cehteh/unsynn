use crate::{Parse, Parser, Result, TokenIter};

/// Conjunctive `A` followed by `B`
pub struct Cons<A: Parser, B: Parser>(A, B);

impl<A: Parser, B: Parser> Parser for Cons<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(A::parser(tokens)?, B::parser(tokens)?))
    }
}

/// Disjunctive `A` or `B` tried in that order.
pub enum Either<A: Parser, B: Parser> {
    /// The first alternative
    First(A),
    /// The second alternative
    Second(B),
}

impl<A: Parser, B: Parser> Parser for Either<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        if let Ok(first) = A::parse(tokens) {
            Ok(Either::First(first))
        } else {
            Ok(Either::Second(B::parser(tokens)?))
        }
    }
}
