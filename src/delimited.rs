use crate::*;

/// A entity `T` followed by a optional delimiting entity 'D'
pub struct Delimited<T: Parse, D: Parse>(pub T, pub Option<D>);

impl<T: Parse, D: Parse> Parse for Delimited<T, D> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(T::parse(tokens)?, Option::<D>::parse(tokens)?))
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
/// `T` followed by an optional '::'
pub type PathSepDelimited<T> = Delimited<T, PathSep>;
