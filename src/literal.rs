//! This module provides a set of literal types that can be used to parse and tokenize
//! literals.  The literals are parsed from the token stream and can be used to represent the
//! parsed value. unsynn defines only simplified literals, such as integers, characters and
//! strings. The literals here are not full rust syntax, which will be defined in the
//! `unsynn-rust` crate.

#![allow(clippy::module_name_repetitions)]

use crate::{Error, Literal, Parser, Result, ToTokens, TokenIter, TokenStream, TokenTree};

/// A simple unsigned 128 bit integer. This is the most simple form to parse integers. Note
/// that only decimal integers without any other characters, signs or suffixes are supported,
/// this is *not* full rust syntax.
#[cfg_attr(feature = "impl_debug", derive(Debug))]
#[derive(Clone)]
pub struct LiteralInteger {
    /// Literal representing an integer
    literal: Literal,
    /// Value of the integer
    value: u128,
}

impl LiteralInteger {
    /// Create a new `LiteralInteger` from a `u128` value.
    #[must_use]
    pub fn new(value: u128) -> Self {
        let literal = Literal::u128_unsuffixed(value);
        Self { literal, value }
    }

    /// Get the value.
    #[must_use]
    pub const fn value(&self) -> u128 {
        self.value
    }

    /// Set to a new the value.
    pub fn set(&mut self, value: u128) {
        *self = Self {
            literal: Literal::u128_unsuffixed(value),
            value,
        };
    }

    /// Deconstructs `self` and gets the `Literal`
    #[must_use]
    pub fn into_inner(self) -> Literal {
        self.literal
    }
}

impl Parser for LiteralInteger {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let value = literal.to_string().parse().map_err(Error::boxed)?;
        Ok(Self { literal, value })
    }
}

impl ToTokens for LiteralInteger {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.literal.to_tokens(tokens);
    }
}

impl PartialEq<u128> for LiteralInteger {
    fn eq(&self, other: &u128) -> bool {
        &self.value == other
    }
}

#[cfg(feature = "impl_display")]
impl std::fmt::Display for LiteralInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A single quoted character literal (`'x'`).
#[cfg_attr(feature = "impl_debug", derive(Debug))]
#[derive(Clone)]
pub struct LiteralCharacter {
    /// Literal representing a single quoted character
    literal: Literal,
    /// The character value
    value: char,
}

impl LiteralCharacter {
    /// Create a new `LiteralCharacter` from a `char` value.
    #[must_use]
    pub fn new(value: char) -> Self {
        let literal = Literal::character(value);
        Self { literal, value }
    }

    /// Get the value.
    #[must_use]
    pub const fn value(&self) -> char {
        self.value
    }

    /// Set to a new value.
    pub fn set(&mut self, value: char) {
        *self = Self {
            literal: Literal::character(value),
            value,
        };
    }

    /// Deconstructs `self` and gets the `Literal`
    #[must_use]
    pub fn into_inner(self) -> Literal {
        self.literal
    }
}

impl Parser for LiteralCharacter {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let string = literal.to_string();
        let mut chars = string.chars();
        // We only need to to check for first single quote, since the lexer already checked
        // for proper literals
        if let (Some('\''), Some(value)) = (chars.next(), chars.next()) {
            Ok(Self { literal, value })
        } else {
            Error::unexpected_token(TokenTree::Literal(literal))
        }
    }
}

impl ToTokens for LiteralCharacter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.literal.to_tokens(tokens);
    }
}

impl PartialEq<char> for LiteralCharacter {
    fn eq(&self, other: &char) -> bool {
        &self.value == other
    }
}

#[cfg(feature = "impl_display")]
impl std::fmt::Display for LiteralCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A double quoted string literal (`"hello"`). The quotes are included in the value.  Note
/// that this is a simplified string literal, and only double quoted strings are supported,
/// this is *not* full rust syntax, eg. byte and C string literals are not supported.
#[cfg_attr(feature = "impl_debug", derive(Debug))]
#[derive(Clone)]
pub struct LiteralString {
    /// Literal representing a double quoted string
    literal: Literal,
    /// The string value
    value: String,
}

impl LiteralString {
    /// Create a new `LiteralString` from a `String` value. The supplied `String` must start
    /// and end with a double quote.
    ///
    /// # Panics
    ///
    /// Panics if the string does not start and end with a double quote.
    #[must_use]
    pub fn new(value: String) -> Self {
        assert!(value.starts_with('"') && value.ends_with('"'));
        let literal = Literal::string(&value);
        Self { literal, value }
    }

    /// Create a new `LiteralString` from a `&str` slice. Adds double quotes around the
    /// supplied string.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Self {
        let value = format!("\"{value}\"");
        let literal = Literal::string(&value);
        Self { literal, value }
    }

    /// Get the `&str` including the surrounding quotes.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the `&str` with the surrounding quotes removed.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value[1..self.value.len() - 1]
    }

    /// Set the value to a new `String`.
    pub fn set(&mut self, value: String) {
        *self = Self {
            literal: Literal::string(&value),
            value,
        };
    }

    /// Deconstructs `self` and gets the `Literal`
    #[must_use]
    pub fn into_inner(self) -> Literal {
        self.literal
    }
}

impl Parser for LiteralString {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let string = literal.to_string();
        // The lexer did its job here as well
        if &string[0..1] == "\"" {
            Ok(Self {
                literal,
                value: string,
            })
        } else {
            Error::unexpected_token(TokenTree::Literal(literal))
        }
    }
}

impl ToTokens for LiteralString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.literal.to_tokens(tokens);
    }
}

impl PartialEq<&str> for LiteralString {
    fn eq(&self, other: &&str) -> bool {
        &self.value == other
    }
}

#[cfg(feature = "impl_display")]
impl std::fmt::Display for LiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// PLANNED: literal!( Type = lit, ...)
