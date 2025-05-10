//! This module contains the transforming parsers. This are the parsers that add, remove,
//! replace or reorder Tokens.

pub use proc_macro2::{Group, Ident, Literal, Punct, TokenStream, TokenTree};

#[allow(clippy::wildcard_imports)]
use crate::*;

use std::marker::PhantomData;
use std::ops::Deref;

/// Succeeds when the next token matches `T`. The token will be removed from the stream but not stored.
/// Consequently the `ToTokens` implementations will panic with a message that it can not be emitted.
/// This can only be used when a token should be present but not stored and never emitted.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "ident ()".to_token_iter();
///
/// let _ = Discard::<Ident>::parse(&mut token_iter).unwrap();
/// assert!(ParenthesisGroup::parse(&mut token_iter).is_ok());
/// ```
#[derive(Clone)]
pub struct Discard<T>(PhantomData<T>);

impl<T: Parse> Parser for Discard<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match T::parser(tokens) {
            Ok(_) => Ok(Self(PhantomData)),
            Err(e) => Err(e),
        }
    }
}

impl<T> ToTokens for Discard<T> {
    #[inline]
    #[mutants::skip]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        unimplemented!("Can not emit tokens for Discard<T>")
    }
}

#[mutants::skip]
impl<T: std::fmt::Debug> std::fmt::Debug for Discard<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Discard<{}>", std::any::type_name::<T>()))
            .finish()
    }
}

/// Skips over expected tokens. Will parse and consume the tokens but not store them.
/// Consequently the `ToTokens` implementations will not output any tokens.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "ident,".to_token_iter();
///
/// let _ = Skip::<Cons<Ident, Comma>>::parse_all(&mut token_iter).unwrap();
/// ```
#[derive(Clone)]
pub struct Skip<T>(PhantomData<T>);

impl<T: Parse> Parser for Skip<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        T::parser(tokens)?;
        Ok(Self(PhantomData))
    }
}

impl<T> ToTokens for Skip<T> {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

#[mutants::skip]
impl<T: std::fmt::Debug> std::fmt::Debug for Skip<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Skip<{}>", std::any::type_name::<T>()))
            .finish()
    }
}

/// Injects tokens without parsing anything.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "foo bar".to_token_iter();
///
/// let parsed = <Cons<Ident, Insert<Plus>, Ident>>::parser(&mut token_iter).unwrap();
/// assert_eq!(parsed.tokens_to_string(), "foo + bar".tokens_to_string());
/// ```
pub struct Insert<T>(pub T);

impl<T: Default> Parser for Insert<T> {
    fn parser(_tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(T::default()))
    }
}

impl<T: ToTokens> ToTokens for Insert<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T> Deref for Insert<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Tries to parse a `T` or inserts a `D` when that fails.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "foo".to_token_iter();
///
/// let parsed = <OrDefault<u32, Question>>::parser(&mut token_iter).unwrap();
/// assert_eq!(parsed.tokens_to_string(), "?".tokens_to_string());
/// ```
pub type OrDefault<T, D> = Either<T, Insert<D>>;

/// Swaps the order of two entities.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "foo bar".to_token_iter();
///
/// let parsed = <Swap<Ident, Ident>>::parser(&mut token_iter).unwrap();
/// assert_eq!(parsed.tokens_to_string(), "bar foo".tokens_to_string());
/// ```
pub struct Swap<A, B>(pub B, pub A);

impl<A: Parse, B: Parse> Parser for Swap<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let a: A = tokens.parse()?;
        let b: B = tokens.parse()?;
        Ok(Self(b, a))
    }
}

impl<A: ToTokens, B: ToTokens> ToTokens for Swap<A, B> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
        self.1.to_tokens(tokens);
    }
}

/// Parse `T` and creates a `LiteralString` from it. When `T` implements `Default`, such as
/// single string (non group) keywords, operators and `Const*` literals. Then this type can be
/// used to create `LiteralStrings` on the fly.
///
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "foo 123".to_token_iter();
///
/// let parsed = <IntoLiteralString<Cons<Ident, LiteralInteger>>>::parser(&mut token_iter).unwrap();
/// assert_eq!(parsed.tokens_to_string(), r#" "foo 123" "#.tokens_to_string());
///
/// keyword!{Foo = "foo"}
/// let default = <IntoLiteralString<Cons<Foo, ConstInteger<1234>>>>::default();
/// assert_eq!(default.tokens_to_string(), r#" "foo 1234" "#.tokens_to_string());
/// ```
pub struct IntoLiteralString<T>(pub LiteralString, PhantomData<T>);

impl<T: Parse + ToTokens> Parser for IntoLiteralString<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(
            LiteralString::from_str(&tokens.parse::<T>()?.tokens_to_string()),
            PhantomData,
        ))
    }
}

impl<T: ToTokens> ToTokens for IntoLiteralString<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: Default + ToTokens> Default for IntoLiteralString<T> {
    fn default() -> Self {
        Self(
            LiteralString::from_str(&T::default().tokens_to_string()),
            PhantomData,
        )
    }
}
