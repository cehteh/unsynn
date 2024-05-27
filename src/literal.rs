use crate::*;

/// A simple unsigned 128 bit integer. Sometimes macros need integers passed. This is the most
/// simple form to parse these. Note that only decimal integers without any other characters,
/// signs or suffixes are supported, this is not full rust syntax.
pub struct LiteralInteger {
    pub literal: Literal,
    pub value: u128,
}

impl Parse for LiteralInteger {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let value = literal.to_string().parse()?;
        Ok(Self { literal, value })
    }
}

/// A single quoted character literal (`'x'`).
pub struct LiteralCharacter {
    pub literal: Literal,
    pub value: char,
}

impl Parse for LiteralCharacter {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let literal = Literal::parser(tokens)?;
        let string = literal.to_string();
        let mut chars = string.chars();
        // We need only to to check for first single quote, since the lexer already checked
        // for proper literals
        if let (Some('\''), Some(value)) = (chars.next(), chars.next()) {
            Ok(Self { literal, value })
        } else {
            Err(format!("Expected a single character literal, got {:?}", literal).into())
        }
    }
}

/// A double quoted string literal (`"hello"`). The quotes are included in the value.
pub struct LiteralString {
    pub literal: Literal,
    pub value: String,
}

impl Parse for LiteralString {
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
            Err(format!("Expected a double quoted string literal, got {:?}", literal).into())
        }
    }
}
