#![doc = include_str!("../README.md")]

pub use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

/// Type alias for the iterator type we use for parsing. This Iterator is Clone and produces
/// `&TokenTree`.
pub type TokenIter = <TokenStream as IntoIterator>::IntoIter;

/// The `Parser` trait that must be implemented by anything we want to parse.  We are parsing
/// over a `proc_macro2::TokenStream` iterator.
pub trait Parser
where
    Self: Sized,
{
    /// The actual parsing function that must be implemented. This mutates the `tokens`
    /// iterator directly without a transaction. This should not be called from user code
    /// except for implementing parsers itself and then only when the rules below are
    /// followed.
    ///
    /// # Implementing Parsers
    ///
    /// The parsers for `TokenStream`, `TokenTree`, `Group`, `Ident`, `Punct`, `Literal`,
    /// `Except` and `Nothing` are the fundamental parsers. Any other parser is composed from
    /// those. This composition is done by calling other `parse()` (or `parser()`)
    /// implementations until eventually one of the above fundamental parsers is called.
    ///
    /// Calling another `parser()` from a `parser()` implementation is only valid when this
    /// is a conjunctive operation and a failure is returned immediately by the `?`
    /// operator. Failing to do so will leave the iterator in a consumed state which breaks
    /// further parsing. When in doubt use `parse()` which is never wrong.
    ///
    /// # Errors
    ///
    /// The `parser()` implementation must return an error if it cannot parse the input. This
    /// error must be a `unsynn::Error`. User code will call `parser()` in a transaction
    /// trough `Parser::parse` which will call the parser in a transaction and roll back on
    /// error.
    fn parser(tokens: &mut TokenIter) -> Result<Self>;
}

/// This trait provides the user facing API to parse grammatical entities. It is implemented
/// for anything that implements the `Parser` trait. The methods here putting the iterator
/// that is used for parsing into a transaction. This iterator is always `Copy`. Instead using
/// a peekable iterator or implementing deeper peeking parsers clone this iterator when
/// necessary, operate on that clone and commit changes back to the original iterator when
/// successful.  This trait cannot be implemented by user code. It is bound to `ToTokens` as
/// well to ensure that everything that can be parsed can be generated as well.
pub trait Parse
where
    Self: Parser + ToTokens,
{
    /// This is the user facing API to parse grammatical entities. Calls a `parser()` within a
    /// transaction. Commits changes on success and returns the parsed value.
    ///
    /// # Errors
    ///
    /// When the parser returns an error the transaction is rolled back and the error is
    /// returned.
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        let result = Self::parser(&mut ptokens)?;
        *tokens = ptokens;
        Ok(result)
    }

    /// Parse a value in a transaction, pass it to a closure which may modify it or return an Error.
    /// When the closure returns an `Ok(Self)` value it is returned.
    ///
    /// # Errors
    ///
    /// When the parser or the closure returns an error, the transaction is rolled back and
    /// the errors is returned.
    fn parse_with(tokens: &mut TokenIter, f: impl FnOnce(Self) -> Result<Self>) -> Result<Self> {
        let mut ptokens = tokens.clone();
        let result = f(Self::parser(&mut ptokens)?)?;
        *tokens = ptokens;
        Ok(result)
    }
}

impl<T: Parser + ToTokens> Parse for T {}

/// We need our own `ToTokens` to be able to implement it for std container types
pub trait ToTokens {
    /// Write `self` to the given `TokenStream`.
    fn to_tokens(&self, tokens: &mut TokenStream);

    /// Convert `self` directly into a `TokenStream` object.
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }
}

// Result and error type
mod error;
pub use error::*;

// various declarative macros
mod macros;

// Parsers for the `proc_macro2` entities and other fundamental types
pub mod fundamental;
#[doc(inline)]
pub use fundamental::*;

// Groups by explicit bracket types
pub mod group;
#[doc(inline)]
pub use group::*;

// Punctuation, delimiters and operators
pub mod punct;
#[doc(inline)]
pub use punct::*;

// Literals
pub mod literal;
#[doc(inline)]
pub use literal::*;

// Delimited sequences
pub mod delimited;
#[doc(inline)]
pub use delimited::*;

// containers and smart pointers
pub mod container;
#[doc(inline)]
pub use container::*;

// combinators
pub mod combinator;
#[doc(inline)]
pub use combinator::*;

mod private {
    pub trait Sealed {}
}
