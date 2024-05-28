#![doc = include_str!("../README.md")]

//use std::fmt::Display;

pub use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

/// Error type for parsing.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
    /// The parsers for `proc_macro2::TokenTree::{self, Group, Ident, Punct, Literal}`,
    /// `Except` and `Nothing` are the fundamental parsers. Any other parser is composed from
    /// those. This composition is done by calling other `parse()` (or `parser()`)
    /// implementations until eventually one of the above fundamental parsers is called.
    ///
    /// Calling another `parser()` from a `parser()` implementation is only valid when this
    /// is a conjunctive operation and a failure is returned immediately by the `?`
    /// operator. Failing to do so will leave the iterator in a consumed state which breaks
    /// further parsing. When in doubt use `parse()` which is never wrong.
    fn parser(tokens: &mut TokenIter) -> Result<Self>;
}

/// This trait provides the user facing API to parse grammatical entities. It is implemented
/// for anything that implements the `Parser` trait. The methods here putting the iterator
/// that is used for parsing into a transaction. This iterator is always `Copy`. Instead using
/// a peekable iterator or implementing deeper peeking parsers clone this iterator when
/// necessary, operate on that clone and commit changes back to the original iterator when
/// successful.  This trait cannot be implemented by user code.
pub trait Parse
where
    Self: Parser + Sized,
{
    /// This is the user facing API to parse grammatical entities. Parses a value within a
    /// transaction. Commits changes on success and returns the parsed value. Rolls the
    /// transaction back on failure and returns an error.
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        let result = Self::parser(&mut ptokens)?;
        *tokens = ptokens;
        Ok(result)
    }

    /// Parse a value in a transaction, pass it to a closure which may modify or drop it.  If
    /// the closure returns `Some`, the value is returned, otherwise the transaction is rolled
    /// back and an error is returned.
    fn parse_with(tokens: &mut TokenIter, f: impl FnOnce(Self) -> Option<Self>) -> Result<Self> {
        let mut ptokens = tokens.clone();
        let result = Self::parser(&mut ptokens)?;
        if let Some(result) = f(result) {
            *tokens = ptokens;
            Ok(result)
        } else {
            Err("condition failed".into())
        }
    }
}

impl<T: Parser> Parse for T {}

// various declarative macros
mod macros;

// Parsers for the `proc_macro2` entities and other fundamental types
mod fundamental;
pub use fundamental::*;

// Groups by explicit bracket types
mod group;
pub use group::*;

// Punctuation, delimiters and operators
mod punct;
pub use punct::*;

// Literals
mod literal;
pub use literal::*;

// Delimited sequences
mod delimited;
pub use delimited::*;

// containers and smart pointers
mod container;
pub use container::*;

// combinators
mod combinator;
pub use combinator::*;

mod private {
    pub trait Sealed {}
}
