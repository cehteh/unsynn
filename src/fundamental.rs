//! This module contains the fundamental parsers. These parsers are the basic tokens from
//! `proc_macro2` and a few other ones defined by unsynn. These are the terminal entities when
//! parsing tokens. Being able to parse [`TokenTree`] and [`TokenStream`] allows one to parse
//! opaque entities where internal details are left out. The [`Cached`] type is used to cache
//! the string representation of the parsed entity. The [`Nothing`] type is used to match
//! without consuming any tokens. The [`Except`] type is used to match when the next token
//! does not match the given type. The [`EndOfStream`] type is used to match the end of the
//! stream when no tokens are left. The [`HiddenState`] type is used to hold additional
//! information that is not part of the parsed syntax.

pub use proc_macro2::{Group, Ident, Literal, Punct, TokenStream, TokenTree};

#[allow(clippy::wildcard_imports)]
use crate::*;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Parses a [`TokenStream`] from the input tokens. This is the primary entity to parse when
/// dealing with opaque entities where internal details are left out.
/// Note that this matches a empty stream (see [`EndOfStream`]) as well.
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

/// Since parsing a [`TokenStream`] succeeds even when no tokens are left, this type is used to
/// parse a [`TokenStream`] that is not empty.
pub struct NonEmptyTokenStream(pub TokenStream);

impl TryFrom<TokenStream> for NonEmptyTokenStream {
    type Error = Error;

    fn try_from(value: TokenStream) -> Result<Self> {
        if value.is_empty() {
            Error::unexpected_end()
        } else {
            Ok(Self(value))
        }
    }
}

impl Parser for NonEmptyTokenStream {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        tokens.parse::<Expect<TokenTree>>()?;
        // A TokenStream will always match, so we can safely unwrap here.
        Ok(Self(TokenStream::parser(tokens).unwrap()))
    }
}

impl ToTokens for NonEmptyTokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone());
    }
}

#[test]
fn test_non_empty_token_stream() {
    let mut token_iter = "ident".to_token_iter();
    let _ = NonEmptyTokenStream::parser(&mut token_iter).unwrap();
}

#[test]
fn test_empty_token_stream() {
    let mut token_iter = "".to_token_iter();
    assert!(NonEmptyTokenStream::parser(&mut token_iter).is_err());
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
        tokens.extend(std::iter::once(self.clone()));
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
        tokens.extend(std::iter::once(TokenTree::Group(self.clone())));
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
        tokens.extend(std::iter::once(TokenTree::Ident(self.clone())));
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
        tokens.extend(std::iter::once(TokenTree::Punct(self.clone())));
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
        tokens.extend(std::iter::once(TokenTree::Literal(self.clone())));
    }
}

/// Getting the underlying source code as string expensive as it always allocates a new
/// `String`. This type caches the string representation of a given entity. Note that this is
/// only reliable for fundamental entities that represent a single token. Spacing between
/// composed tokens is not stable and should be considered informal only.
///
/// # Example
///
/// ```
/// use unsynn::*;
/// let mut token_iter = "ident 1234".to_token_iter();
///
/// let cached_ident = Cached::<Ident>::parse(&mut token_iter).unwrap();
/// assert!(cached_ident == "ident");
/// ```
#[derive(Clone)]
pub struct Cached<T> {
    value: T,
    string: String,
}

impl<T: Parse + ToTokens> Parser for Cached<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let value = T::parser(tokens)?;
        let string = value.tokens_to_string();
        Ok(Self { value, string })
    }
}

impl<T: ToTokens> ToTokens for Cached<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.value.to_tokens(tokens);
    }
}

impl<T: ToTokens> Cached<T> {
    /// Sets the value and updates the string representation.
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.string = self.value.tokens_to_string();
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

impl<T> Deref for Cached<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> PartialEq<&str> for Cached<T> {
    fn eq(&self, other: &&str) -> bool {
        &self.string == other
    }
}

impl<T> AsRef<T> for Cached<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsRef<str> for Cached<T> {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

#[cfg(feature = "impl_debug")]
impl<T: std::fmt::Debug> std::fmt::Debug for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Cached<{}>", std::any::type_name::<T>()))
            .field("value", &self.value)
            .field("string", &self.string)
            .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<T: std::fmt::Display> std::fmt::Display for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

/// Convert a `Cached<T: Into<TokenTree>>` object into a `TokenTree`.
impl<T: std::fmt::Display + Into<TokenTree>> From<Cached<T>> for TokenTree {
    fn from(cached: Cached<T>) -> Self {
        cached.value.into()
    }
}

#[test]
fn test_cached_into_tt() {
    let mut token_iter = "ident".to_token_iter();
    let ident = Cached::<Ident>::parser(&mut token_iter).unwrap();
    let _: TokenTree = ident.into();
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
/// to parse a [`Repeats`] without a delimiter.  Note that using `Nothing` as primary entity
/// in a [`Vec`], [`LazyVec`], [`DelimitedVec`] or [`Repeats`] will result in an infinite
/// loop.
#[cfg_attr(feature = "impl_debug", derive(Debug))]
#[derive(Clone)]
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

#[cfg(feature = "impl_display")]
impl std::fmt::Display for Nothing {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// A unit that always fails to match. This is useful as default for generics.
/// See how [`Either<A, B, C, D>`] uses this for unused alternatives.
#[cfg_attr(feature = "impl_debug", derive(Debug))]
#[derive(Clone)]
pub struct Invalid;

impl Parser for Invalid {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Error::unexpected_token(tokens.clone().next().unwrap())
    }
}

impl ToTokens for Invalid {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

#[cfg(feature = "impl_display")]
impl std::fmt::Display for Invalid {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Succeeds when the next token does not match `T`. Will not consume any tokens.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "ident".to_token_iter();
///
/// let _ = Except::<Punct>::parser(&mut token_iter).unwrap();
/// ```
#[derive(Clone)]
pub struct Except<T>(PhantomData<T>);

impl<T: Parse> Parser for Except<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match T::parser(&mut ptokens) {
            Ok(_) => Error::unexpected_token(tokens.clone().next().unwrap()),
            Err(_) => Ok(Self(PhantomData)),
        }
    }
}

impl<T> ToTokens for Except<T> {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

#[cfg(feature = "impl_debug")]
impl<T: std::fmt::Debug> std::fmt::Debug for Except<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Except<{}>", std::any::type_name::<T>()))
            .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<T> std::fmt::Display for Except<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Succeeds when the next token would match `T`. Will not consume any tokens.
/// This is similar to peeking.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "ident".to_token_iter();
///
/// let _ = Expect::<Ident>::parser(&mut token_iter).unwrap();
/// ```
#[derive(Clone)]
pub struct Expect<T>(PhantomData<T>);

impl<T: Parse> Parser for Expect<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match T::parser(&mut ptokens) {
            Ok(_) => Ok(Self(PhantomData)),
            Err(e) => Err(e),
        }
    }
}

impl<T> ToTokens for Expect<T> {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

#[cfg(feature = "impl_debug")]
impl<T: std::fmt::Debug> std::fmt::Debug for Expect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Expect<{}>", std::any::type_name::<T>()))
            .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<T> std::fmt::Display for Expect<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Matches the end of the stream when no tokens are left.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = "".to_token_iter();
///
/// let _end_ = EndOfStream::parser(&mut token_iter).unwrap();
/// ```
#[cfg_attr(feature = "impl_debug", derive(Debug))]
#[derive(Clone)]
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

/// Sometimes one want to compose types or create structures for unsynn that have members that
/// are not part of the parsed syntax but add some additional information. This struct can be
/// used to hold such members while still using the `Parser` and `ToTokens` trait
/// implementations automatically generated by the `unsynn!{}` macro or composition syntax.
/// `HiddenState` will not consume any tokens when parsing and will not emit any tokens when
/// generating a `TokenStream`. On parsing it is initialized with a default value. It has
/// `Deref` and `DerefMut` implemented to access the inner value.
#[derive(Clone)]
pub struct HiddenState<T: Default>(T);

impl<T: Default> Deref for HiddenState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> DerefMut for HiddenState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Default> Parser for HiddenState<T> {
    #[inline]
    fn parser(_ctokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(T::default()))
    }
}

impl<T: Default> ToTokens for HiddenState<T> {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        /*NOP*/
    }
}

impl<T: Default> Default for HiddenState<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(feature = "impl_debug")]
impl<T: Default + std::fmt::Debug> std::fmt::Debug for HiddenState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("HiddenState<{}>", std::any::type_name::<T>()))
            .field(&self.0)
            .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<T: Default> std::fmt::Display for HiddenState<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
