use crate::*;

use std::marker::PhantomData;

/// A followed by B
pub struct Cons<A:Parse,B: Parse>(A,B);

impl<A: Parse, B: Parse> Parse for Cons<A,B> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        let result = Self(A::parse(&mut ptokens)?, B::parse(&mut ptokens)?);
        *tokens = ptokens;
        Ok(result)
    }
}

/// Succeeds when the next token does not match T. Will not consume any tokens.
pub struct Except<T:Parse>(PhantomData<T>);

impl<T: Parse> Parse for Except<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match T::parse(&mut ptokens) {
            Ok(_) => Err(format!("unexpected {}", std::any::type_name::<T>()).into()),
            Err(_) => {
                Ok(Self(PhantomData))
            }
        }
    }
}


/// Either A or B in that order.
pub enum Either<A:Parse,B: Parse> {
    First(A),
    Second(B),
}

impl<A: Parse, B: Parse> Parse for Either<A, B> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        if let Ok(first) = A::parse(&mut ptokens){
            *tokens = ptokens;
            Ok(Either::First(first))
        } else if let Ok(second) = B::parse(&mut ptokens){
            *tokens = ptokens;
            Ok(Either::Second(second))
        } else {
            Err(format!("neither of Either<{}, {}> matched",
                        std::any::type_name::<A>(),
                        std::any::type_name::<B>()
            ).into())
        }
    }
}

