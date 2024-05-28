use crate::*;

use std::marker::PhantomData;

/// A followed by B
pub struct Cons<A: Parser, B: Parser>(A, B);

impl<A: Parser, B: Parser> Parser for Cons<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(A::parser(tokens)?, B::parser(tokens)?))
    }
}

/// Succeeds when the next token does not match T. Will not consume any tokens.
pub struct Except<T: Parser>(PhantomData<T>);

impl<T: Parser> Parser for Except<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match T::parser(&mut ptokens) {
            Ok(_) => Err(format!("unexpected {}", std::any::type_name::<T>()).into()),
            Err(_) => Ok(Self(PhantomData)),
        }
    }
}

/// Either A or B in that order.
pub enum Either<A: Parser, B: Parser> {
    First(A),
    Second(B),
}

impl<A: Parser, B: Parser> Parser for Either<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        if let Ok(first) = A::parse(tokens) {
            Ok(Either::First(first))
        } else if let Ok(second) = B::parser(tokens) {
            Ok(Either::Second(second))
        } else {
            Err(format!(
                "neither of Either<{}, {}> matched",
                std::any::type_name::<A>(),
                std::any::type_name::<B>()
            )
            .into())
        }
    }
}
