//! For easier composition we define the `Delimited` type here which is a `T` followed by a
//! optional delimiting entity `D`. This is used by the `DelimitedVec` type to parse a list of
//! entities separated by a delimiter.

#![allow(clippy::module_name_repetitions)]

use crate::{
    Colon, Comma, Dot, Parse, Parser, PathSep, Result, Semicolon, ToTokens, TokenIter, TokenStream,
};

/// A entity `T` followed by a optional delimiting entity `D`
pub struct Delimited<T: Parse, D: Parse>(pub T, pub Option<D>);

impl<T: Parse, D: Parse> Parser for Delimited<T, D> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(T::parser(tokens)?, Option::<D>::parser(tokens)?))
    }
}

impl<T: Parse, D: Parse> ToTokens for Delimited<T, D> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
        self.1.to_tokens(tokens);
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
