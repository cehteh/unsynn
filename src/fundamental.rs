use crate::*;

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;

impl Parser for TokenTree {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(token) => Ok(token),
            None => Err("expected TokenTree, got end of stream".into()),
        }
    }
}

impl Parser for Group {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            Some(other) => Err(format!(
                "expected Group, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Group, got end of stream".into()),
        }
    }
}

impl Parser for Ident {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            Some(other) => Err(format!(
                "expected Ident, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Ident, got end of stream".into()),
        }
    }
}

impl Parser for Punct {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct)) => Ok(punct),
            Some(other) => Err(format!(
                "expected Punct, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Punct, got end of stream".into()),
        }
    }
}

impl Parser for Literal {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Literal(literal)) => Ok(literal),
            Some(other) => Err(format!(
                "expected Literal, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Literal, got end of stream".into()),
        }
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
pub struct Cached<T: Parser + ToString> {
    value: T,
    string: String,
}

impl<T: Parser + ToString> Parser for Cached<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let value = T::parser(tokens)?;
        let string = value.to_string();
        Ok(Self { value, string })
    }
}

impl<T: Parser + ToString> Cached<T> {
    /// Sets the value and updates the string representation.
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.string = self.value.to_string();
    }

    /// Deconstructs self and returns the inner value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: Parser + ToString> Deref for Cached<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Parser + ToString + Clone> Clone for Cached<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            string: self.string.clone(),
        }
    }
}

impl<T: Parser + ToString> PartialEq<&str> for Cached<T> {
    fn eq(&self, other: &&str) -> bool {
        &self.string == other
    }
}

impl<T: Parser + ToString> Display for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<T: Parser + ToString> AsRef<T> for Cached<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Parser + ToString> AsRef<str> for Cached<T> {
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
/// to parse a Repetition without a delimiter.  Note that using `Nothing` as primary entity in
/// a `Vec`, `DelimitedVec` or `Repetition` will result in an infinite loop.
pub struct Nothing;

impl Parser for Nothing {
    fn parser(_tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self)
    }
}

impl Display for Nothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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

/// Matches the end of the stream when no tokens are left
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = quote::quote! { }.into_iter();
///
/// let _eos = EndOfStream::parser(&mut token_iter).unwrap();
/// ```
pub struct EndOfStream;

impl Parser for EndOfStream {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            None => Ok(Self),
            Some(next) => Err(format!("expected end of file, found {next:?}").into()),
        }
    }
}
