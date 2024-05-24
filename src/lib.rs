#![doc = include_str!("../README.md")]

//use std::fmt::Display;

pub use proc_macro2::*;

/// Error type for parsing.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Type alias for the iterator type we use for parsing. This Iterator is Clone and produces
/// `&TokenTree`.  Peeking requires to clone the Iterator and commit changes by copying the
/// clone to the original.
pub type TokenIter = <TokenStream as IntoIterator>::IntoIter;

/// The parser trait that must be implemented by anything we want to parse
pub trait Parse //: Debug + Display + ToTokens
where
    Self: Sized,
{
    /// Parse a value from the TokenIter. Must not advance `tokens` when a parse error occurs.
    fn parse(tokens: &mut TokenIter) -> Result<Self>;
}

// Parsers for the `proc_macro2` entities
mod procmacro;

// Groups by explicit bracket types
mod group;
pub use group::*;

// Punctuation, delimiters and operators
mod punct;
pub use punct::*;

// Delimited sequences
mod delimited;
pub use delimited::*;

// containers and smart pointers
mod container;
pub use container::*;

// combinators
mod combinator;
pub use combinator::*;

mod sealed {
    pub trait Sealed {}
}
