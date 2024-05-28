use crate::*;

/// A entity `T` followed by a optional delimiting entity `D`
pub struct Delimited<T: Parse, D: Parse>(pub T, pub Option<D>);

impl<T: Parse, D: Parse> Parser for Delimited<T, D> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(T::parser(tokens)?, Option::<D>::parser(tokens)?))
    }
}

/// `T` followed by an optional `,`
pub type CommaDelimited<T> = Delimited<T, Comma>;
/// `T` followed by an optional `:`
pub type ColonDelimited<T> = Delimited<T, Colon>;
/// `T` followed by an optional `;`
pub type SemicolonDelimited<T> = Delimited<T, Semicolon>;
/// `T` followed by an optional `.`
pub type DotDelimited<T> = Delimited<T, Dot>;
/// `T` followed by an optional `::`
pub type PathSepDelimited<T> = Delimited<T, PathSep>;
