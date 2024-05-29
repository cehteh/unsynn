#![allow(clippy::module_name_repetitions)]

use crate::{Error, Parser, Result, Spacing, TokenIter, TokenTree};
use std::fmt::Display;

/// A single character punctuation token lexed with `Spacing::Alone`.
pub struct OnePunct<const C: char>;

impl<const C: char> OnePunct<C> {
    /// Get the `char` value this object represents.
    #[must_use]
    pub fn as_char(&self) -> char {
        C
    }
}

impl<const C: char> Parser for OnePunct<C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct))
                if punct.spacing() == Spacing::Alone && punct.as_char() == C =>
            {
                Ok(Self)
            }
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl<const C: char> Display for OnePunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

/// A single character punctuation token where the lexer joined it with the next `Punct` or a
/// single quote followed by a identifier (rust lifetime). Note that the rust lexer knows
/// about rust operators, the rules when `Punct` are `Spacing::Alone` or `Spacing::Joint` are
/// geared towards rust syntax.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = quote::quote! {:::}.into_iter();
///
/// // The lexer won't join ':::' together it only knows about '::'
/// let colon = JointPunct::<':'>::parse(&mut token_iter).unwrap();
/// let colon = OnePunct::<':'>::parse(&mut token_iter).unwrap();
/// let colon = OnePunct::<':'>::parse(&mut token_iter).unwrap();
/// ```
pub struct JointPunct<const C: char>;

impl<const C: char> JointPunct<C> {
    /// Get the `char` value this object represents.
    #[must_use]
    pub fn as_char(&self) -> char {
        C
    }
}

impl<const C: char> Parser for JointPunct<C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct))
                if punct.spacing() == Spacing::Joint && punct.as_char() == C =>
            {
                Ok(Self)
            }
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl<const C: char> Display for JointPunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

/// Double character joint punctuation.
pub struct TwoPunct<const C1: char, const C2: char>;

impl<const C1: char, const C2: char> Parser for TwoPunct<C1, C2> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match (tokens.next(), tokens.next()) {
            (Some(TokenTree::Punct(c1)), Some(TokenTree::Punct(c2)))
                if c1.spacing() == Spacing::Joint
                    && c1.as_char() == C1
                    && c2.spacing() == Spacing::Alone
                    && c2.as_char() == C2 =>
            {
                Ok(Self)
            }
            (Some(other), _) => Error::unexpected_token(other),
            (None, _) => Error::unexpected_end(),
        }
    }
}

impl<const C1: char, const C2: char> Display for TwoPunct<C1, C2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C1}{C2}")
    }
}

/// Triple character joint punctuation.
pub struct ThreePunct<const C1: char, const C2: char, const C3: char>;

impl<const C1: char, const C2: char, const C3: char> Parser for ThreePunct<C1, C2, C3> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match (tokens.next(), tokens.next(), tokens.next()) {
            (
                Some(TokenTree::Punct(c1)),
                Some(TokenTree::Punct(c2)),
                Some(TokenTree::Punct(c3)),
            ) if c1.spacing() == Spacing::Joint
                && c1.as_char() == C1
                && c2.spacing() == Spacing::Joint
                && c2.as_char() == C2
                && c3.spacing() == Spacing::Alone
                && c3.as_char() == C3 =>
            {
                Ok(Self)
            }
            (Some(other), _, _) => Error::unexpected_token(other),
            (None, _, _) => Error::unexpected_end(),
        }
    }
}

impl<const C1: char, const C2: char, const C3: char> Display for ThreePunct<C1, C2, C3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C1}{C2}{C3}")
    }
}

// unsynn policy is not to implement rust grammar, for the Punct tokens we make an exception
// because they are mostly universal and already partial lexed (Spacing::Alone/Joint) it would
// add a lot confusion when every grammar has to redefine its own Punct types.

/// `+`
pub type Plus = OnePunct<'+'>;
/// `-`
pub type Minus = OnePunct<'-'>;
/// `*`
pub type Star = OnePunct<'*'>;
/// `/`
pub type Slash = OnePunct<'/'>;
/// `%`
pub type Percent = OnePunct<'%'>;
/// `^`
pub type Caret = OnePunct<'^'>;
/// `!`
pub type Bang = OnePunct<'!'>;
/// `&`
pub type And = OnePunct<'&'>;
/// `|`
pub type Or = OnePunct<'|'>;
/// `&&`
pub type AndAnd = TwoPunct<'&', '&'>;
/// `||`
pub type OrOr = TwoPunct<'|', '|'>;
/// `<<`
pub type Shl = TwoPunct<'<', '<'>;
/// `>>`
pub type Shr = TwoPunct<'>', '>'>;
/// `+=`
pub type PlusEq = TwoPunct<'+', '='>;
/// `-=`
pub type MinusEq = TwoPunct<'-', '='>;
/// `*=`
pub type StarEq = TwoPunct<'*', '='>;
/// `/=`
pub type SlashEq = TwoPunct<'/', '='>;
/// `%=`
pub type PercentEq = TwoPunct<'%', '='>;
/// `^=`
pub type CaretEq = TwoPunct<'^', '='>;
/// `&=`
pub type AndEq = TwoPunct<'&', '='>;
/// `|=`
pub type OrEq = TwoPunct<'|', '='>;
/// `<<=`
pub type ShlEq = ThreePunct<'<', '<', '='>;
/// `>>=`
pub type ShrEq = ThreePunct<'>', '>', '='>;
/// `=`
pub type Assign = OnePunct<'='>;
/// `==`
pub type Equal = TwoPunct<'=', '='>;
/// `!=`
pub type NotEqual = TwoPunct<'!', '='>;
/// `>`
pub type Gt = OnePunct<'>'>;
/// `<`
pub type Lt = OnePunct<'<'>;
/// `>=`
pub type Ge = TwoPunct<'>', '='>;
/// `<=`
pub type Le = TwoPunct<'<', '='>;
/// `@`
pub type At = OnePunct<'@'>;
/// `_`
pub type Underscore = OnePunct<'_'>;
/// `.`
pub type Dot = OnePunct<'.'>;
/// `..`
pub type DotDot = TwoPunct<'.', '.'>;
/// `...`
pub type Ellipsis = ThreePunct<'.', '.', '.'>;
/// `..=`
pub type DotDotEq = ThreePunct<'.', '.', '='>;
/// `,`
pub type Comma = OnePunct<','>;
/// `;`
pub type Semicolon = OnePunct<';'>;
/// `:`
pub type Colon = OnePunct<':'>;
/// `::`
pub type PathSep = TwoPunct<':', ':'>;
/// `->`
pub type RArrow = TwoPunct<'-', '>'>;
/// `=>`
pub type FatArrow = TwoPunct<'=', '>'>;
/// `<-`
pub type LArrow = TwoPunct<'<', '-'>;
/// `#`
pub type Pound = OnePunct<'#'>;
/// `$`
pub type Dollar = OnePunct<'$'>;
/// `?`
pub type Question = OnePunct<'?'>;
/// `~`
pub type Tilde = OnePunct<'~'>;
/// `\\`
pub type Backslash = OnePunct<'\\'>;
