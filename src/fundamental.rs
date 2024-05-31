use crate::{
    Error, Group, Ident, Literal, Parse, Parser, Punct, Result, ToTokens, TokenIter, TokenStream,
    TokenTree,
};

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;

impl Parser for TokenStream {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut output = TokenStream::new();
        output.extend(tokens);
        Ok(output)
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.clone());
    }
}

impl Parser for TokenTree {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(token) => Ok(token),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for TokenTree {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, tokens);
    }
}

impl Parser for Group {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for Group {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, tokens);
    }
}

impl Parser for Ident {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for Ident {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, tokens);
    }
}

impl Parser for Punct {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct)) => Ok(punct),
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for Punct {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, tokens);
    }
}

impl Parser for Literal {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Literal(literal)) => Ok(literal),
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for Literal {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, tokens);
    }
}

/// Getting the underlying source code as string from a parser is expensive. This type
/// caches the string representation given entity.
///
/// # Example
///
/// ```
/// use unsynn::*;
/// let mut token_iter = quote::quote! {ident 1234}.into_iter();
///
/// let cached_ident = Cached::<Ident>::parse(&mut token_iter).unwrap();
/// assert!(cached_ident == "ident");
/// ```
#[derive(Debug)]
pub struct Cached<T: Parse + ToString> {
    value: T,
    string: String,
}

impl<T: Parse + ToString> Parser for Cached<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let value = T::parser(tokens)?;
        let string = value.to_string();
        Ok(Self { value, string })
    }
}

impl<T: Parse + ToString> ToTokens for Cached<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.value.to_tokens(tokens);
    }
}

impl<T: Parse + ToString> Cached<T> {
    /// Sets the value and updates the string representation.
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.string = self.value.to_string();
    }

    /// Deconstructs self and returns the inner value.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Gets the cached string representation
    pub fn string(&self) -> &str {
        &self.string
    }
}

impl<T: Parse + ToString> Deref for Cached<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Parse + ToString> PartialEq<&str> for Cached<T> {
    fn eq(&self, other: &&str) -> bool {
        &self.string == other
    }
}

impl<T: Parse + ToString> Display for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<T: Parse + ToString> AsRef<T> for Cached<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Parse + ToString> AsRef<str> for Cached<T> {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

/// `TokenTree` with cached string representation.
pub type CachedTokenTree = Cached<TokenTree>;
/// `Group` with cached string representation.
pub type CachedGroup = Cached<Group>;
/// `Ident` with cached string representation.
pub type CachedIdent = Cached<Ident>;
/// `Punct` with cached string representation.
pub type CachedPunct = Cached<Punct>;
/// `Literal` with cached string representation.
pub type CachedLiteral = Cached<Literal>;

/// A unit that always matches without consuming any tokens.  This is required when one wants
/// to parse a `Repetition` without a delimiter.  Note that using `Nothing` as primary entity in
/// a `Vec`, `DelimitedVec` or `Repetition` will result in an infinite loop.
pub struct Nothing;

impl Parser for Nothing {
    #[inline]
    fn parser(_tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self)
    }
}

impl ToTokens for Nothing {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

impl Display for Nothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// Succeeds when the next token does not match T. Will not consume any tokens.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = quote::quote! {ident}.into_iter();
///
/// let _ = Except::<Punct>::parser(&mut token_iter).unwrap();
/// ```
pub struct Except<T: Parse>(PhantomData<T>);

impl<T: Parse> Parser for Except<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match T::parser(&mut ptokens) {
            Ok(_) => Error::unexpected_token(tokens.clone().next().unwrap()),
            Err(_) => Ok(Self(PhantomData)),
        }
    }
}

impl<T: Parse> ToTokens for Except<T> {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

/// Matches the end of the stream when no tokens are left
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = quote::quote!{ }.into_iter();
///
/// let _end_ = EndOfStream::parser(&mut token_iter).unwrap();
/// ```
pub struct EndOfStream;

impl Parser for EndOfStream {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            None => Ok(Self),
            Some(next) => Error::unexpected_token(next),
        }
    }
}

impl ToTokens for EndOfStream {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}
