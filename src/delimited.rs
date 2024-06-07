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

#[cfg(feature = "impl_debug")]
impl<T: Parse + std::fmt::Debug, D: Parse + std::fmt::Debug> std::fmt::Debug for Delimited<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple(&format!(
            "Delimited<{}, {}>",
            std::any::type_name::<T>(),
            std::any::type_name::<D>()
        ))
        .field(&self.0)
        .field(&self.1)
        .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<T: Parse + std::fmt::Display, D: Parse + std::fmt::Display> std::fmt::Display
    for Delimited<T, D>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.0,
            self.1
                .as_ref()
                // use a space when there is no delimiter otherwise tokens it would be
                // indistinguishable where one token ends and the next begins.
                .map_or(String::from(" "), ToString::to_string)
        )
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
