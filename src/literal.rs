#![allow(clippy::module_name_repetitions)]

use crate::{Literal, Parser, Result, TokenIter};

/// A simple unsigned 128 bit integer. This is the most simple form to parse integers. Note
/// that only decimal integers without any other characters, signs or suffixes are supported,
/// this is *not* full rust syntax.
pub struct LiteralInteger {
    /// Literal representing an integer
    pub literal: Literal,
    /// Value of the integer
    pub value: u128,
}

impl Parser for LiteralInteger {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let value = literal.to_string().parse()?;
        Ok(Self { literal, value })
    }
}

/// A single quoted character literal (`'x'`).
pub struct LiteralCharacter {
    /// Literal representing a single quoted character
    pub literal: Literal,
    /// The character value
    pub value: char,
}

impl Parser for LiteralCharacter {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let string = literal.to_string();
        let mut chars = string.chars();
        // We need only to to check for first single quote, since the lexer already checked
        // for proper literals
        if let (Some('\''), Some(value)) = (chars.next(), chars.next()) {
            Ok(Self { literal, value })
        } else {
            Err(format!("Expected a single character literal, got {literal:?}").into())
        }
    }
}

/// A double quoted string literal (`"hello"`). The quotes are included in the value.
pub struct LiteralString {
    /// Literal representing a double quoted string
    pub literal: Literal,
    /// The string value
    pub value: String,
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
            Err(format!("Expected a double quoted string literal, got {literal:?}").into())
        }
    }
}
