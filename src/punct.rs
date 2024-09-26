//! This module contains types for punctuation tokens. These are used to represent single and
//! multi character punctuation tokens. For single character punctuation tokens, there are
//! there are [`PunctAny`], [`PunctAlone`] and [`PunctJoint`] types.
//! Combined punctuation tokens are represented by [`Operator`]. The [`operator!`] macro can be
//! used to define custom operators.
#![allow(clippy::module_name_repetitions)]

pub use proc_macro2::Spacing;

use crate::{operator, Error, Parser, Punct, Result, ToTokens, TokenIter, TokenStream, TokenTree};

/// A single character punctuation token with any kind of [`Spacing`],
#[derive(Default, Clone)]
pub struct PunctAny<const C: char>;

impl<const C: char> Parser for PunctAny<C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == C => Ok(Self),
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl<const C: char> ToTokens for PunctAny<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Punct::new(C, Spacing::Alone).to_tokens(tokens);
    }
}

/// Convert a `PunctAny` object into a `TokenTree`.
impl<const C: char> From<PunctAny<C>> for TokenTree {
    fn from(_: PunctAny<C>) -> Self {
        TokenTree::Punct(Punct::new(C, Spacing::Alone))
    }
}

#[cfg(feature = "impl_display")]
impl<const C: char> std::fmt::Display for PunctAny<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

#[cfg(any(debug_assertions, feature = "impl_debug"))]
impl<const C: char> std::fmt::Debug for PunctAny<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PunctAny<{C:?}>")
    }
}

/// A single character punctuation token where the lexer joined it with the next `Punct` or a
/// single quote followed by a identifier (rust lifetime).
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// // The TokenStream::from_str() keeps the ':::'
/// let mut token_iter = ":::".to_token_iter();
///
/// let colon = PunctJoint::<':'>::parse(&mut token_iter).unwrap();
/// let colon = PunctJoint::<':'>::parse(&mut token_iter).unwrap();
/// let colon = PunctAny::<':'>::parse(&mut token_iter).unwrap();
///
/// // Caveat: The quote! macro won't join ':::' together
/// // let mut token_iter = quote::quote! {:::}.into_iter();
/// //
/// // let colon = PunctJoint::<':'>::parse(&mut token_iter).unwrap();
/// // let colon = PunctAny::<':'>::parse(&mut token_iter).unwrap();
/// // let colon = PunctAny::<':'>::parse(&mut token_iter).unwrap();
/// ```
#[derive(Default, Clone)]
pub struct PunctJoint<const C: char>;

impl<const C: char> PunctJoint<C> {
    /// Create a new [`PunctJoint`] object.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Get the `char` value this object represents.
    #[must_use]
    pub const fn as_char(&self) -> char {
        C
    }
}

impl<const C: char> Parser for PunctJoint<C> {
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

impl<const C: char> ToTokens for PunctJoint<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Punct::new(C, Spacing::Joint).to_tokens(tokens);
    }
}

#[cfg(feature = "impl_display")]
impl<const C: char> std::fmt::Display for PunctJoint<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

#[cfg(any(debug_assertions, feature = "impl_debug"))]
impl<const C: char> std::fmt::Debug for PunctJoint<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PunctJoint<{C:?}>")
    }
}

/// Convert a [`PunctJoint`] object into a [`TokenTree`].
impl<const C: char> From<PunctJoint<C>> for TokenTree {
    fn from(_: PunctJoint<C>) -> Self {
        TokenTree::Punct(Punct::new(C, Spacing::Joint))
    }
}

#[test]
fn test_joint_punct_into_tt() {
    let mut token_iter = "+=".to_token_iter();
    let plus = PunctJoint::<'+'>::parser(&mut token_iter).unwrap();
    assert_eq!(plus.as_char(), '+');
    let _: TokenTree = plus.into();
}

/// A single character punctuation token which is not followed by another punctuation
/// character.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = ": :".to_token_iter();
///
/// let colon = PunctAlone::<':'>::parse(&mut token_iter).unwrap();
/// let colon = PunctAlone::<':'>::parse(&mut token_iter).unwrap();
/// ```
#[derive(Default, Clone)]
pub struct PunctAlone<const C: char>;

impl<const C: char> PunctAlone<C> {
    /// Create a new [`PunctAlone`] object.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Get the `char` value this object represents.
    #[must_use]
    pub const fn as_char(&self) -> char {
        C
    }
}

impl<const C: char> Parser for PunctAlone<C> {
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

impl<const C: char> ToTokens for PunctAlone<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Punct::new(C, Spacing::Alone).to_tokens(tokens);
    }
}

#[cfg(feature = "impl_display")]
impl<const C: char> std::fmt::Display for PunctAlone<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

#[cfg(any(debug_assertions, feature = "impl_debug"))]
impl<const C: char> std::fmt::Debug for PunctAlone<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PunctAlone<{C:?}>")
    }
}

/// Convert a [`PunctAlone`] object into a [`TokenTree`].
impl<const C: char> From<PunctAlone<C>> for TokenTree {
    fn from(_: PunctAlone<C>) -> Self {
        TokenTree::Punct(Punct::new(C, Spacing::Alone))
    }
}

#[test]
fn test_alone_punct_into_tt() {
    let mut token_iter = "+ +".to_token_iter();
    let plus = PunctAlone::<'+'>::parser(&mut token_iter).unwrap();
    assert_eq!(plus.as_char(), '+');
    let _: TokenTree = plus.into();
}

/// Operators made from up to four ASCII punctuation characters. Unused characters default to `\0`.
/// Custom operators can be defined with the [`operator!`] macro. All but the last character are
/// [`Spacing::Joint`]. Attention must be payed when operators have the same prefix, the shorter
/// ones need to be tried first.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Operator<
    const C1: char,
    const C2: char = '\0',
    const C3: char = '\0',
    const C4: char = '\0',
>;

impl<const C1: char, const C2: char, const C3: char, const C4: char> Operator<C1, C2, C3, C4> {
    /// Create a new `Operator` object.
    #[must_use]
    pub const fn new() -> Self {
        const {
            assert!(C1.is_ascii_punctuation());
            assert!(C2 == '\0' || C2.is_ascii_punctuation());
            assert!(C3 == '\0' || C3.is_ascii_punctuation());
            assert!(C4 == '\0' || C4.is_ascii_punctuation());
        }
        Self
    }
}

impl<const C1: char, const C2: char, const C3: char, const C4: char> Parser
    for Operator<C1, C2, C3, C4>
{
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        if C2 == '\0' {
            PunctAny::<C1>::parser(tokens)?;
            Ok(Self)
        } else {
            PunctJoint::<C1>::parser(tokens)?;
            if C3 == '\0' {
                PunctAny::<C2>::parser(tokens)?;
                Ok(Self)
            } else {
                PunctJoint::<C2>::parser(tokens)?;
                if C4 == '\0' {
                    PunctAny::<C3>::parser(tokens)?;
                    Ok(Self)
                } else {
                    PunctJoint::<C3>::parser(tokens)?;
                    PunctAny::<C4>::parser(tokens)?;
                    Ok(Self)
                }
            }
        }
    }
}

impl<const C1: char, const C2: char, const C3: char, const C4: char> ToTokens
    for Operator<C1, C2, C3, C4>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Make the spacing `Joint` when the next character is not a space.
        const fn spacing(c: char) -> Spacing {
            if c == '\0' {
                Spacing::Alone
            } else {
                Spacing::Joint
            }
        }

        Punct::new(C1, spacing(C2)).to_tokens(tokens);
        if C2 != '\0' {
            Punct::new(C2, spacing(C3)).to_tokens(tokens);
            if C3 != '\0' {
                Punct::new(C3, spacing(C4)).to_tokens(tokens);
                if C4 != '\0' {
                    Punct::new(C4, Spacing::Alone).to_tokens(tokens);
                }
            }
        };
    }
}

#[cfg(feature = "impl_display")]
impl<const C1: char, const C2: char, const C3: char, const C4: char> std::fmt::Display
    for Operator<C1, C2, C3, C4>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if C4 != '\0' {
            write!(f, "{C1}{C2}{C3}{C4}")
        } else if C3 != '\0' {
            write!(f, "{C1}{C2}{C3}")
        } else if C2 != '\0' {
            write!(f, "{C1}{C2}")
        } else {
            write!(f, "{C1}")
        }
    }
}

#[cfg(any(debug_assertions, feature = "impl_debug"))]
impl<const C1: char, const C2: char, const C3: char, const C4: char> std::fmt::Debug
    for Operator<C1, C2, C3, C4>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if C4 != '\0' {
            write!(f, "Operator<'{C1}{C2}{C3}{C4}'>")
        } else if C3 != '\0' {
            write!(f, "Operator<'{C1}{C2}{C3}'>")
        } else if C2 != '\0' {
            write!(f, "Operator<'{C1}{C2}'>")
        } else {
            write!(f, "Operator<'{C1}'>")
        }
    }
}

// unsynn policy is not to implement rust grammar, for the Punct tokens we make an exception
// because they are mostly universal and already partial lexed (Spacing::Alone/Joint) it would
// add a lot confusion when every grammar has to redefine its own Punct types.

/// `'` With `Spacing::Joint`
pub type LifetimeTick = PunctJoint<'\''>;

operator! {
    /// `+`
    pub Plus = "+";
    /// `-`
    pub Minus = "-";
    /// `*`
    pub Star = "*";
    /// `/`
    pub Slash = "/";
    /// `%`
    pub Percent = "%";
    /// `^`
    pub Caret = "^";
    /// `!`
    pub Bang = "!";
    /// `&`
    pub And = "&";
    /// `|`
    pub Or = "|";
    /// `&&`
    pub AndAnd = "&&";
    /// `||`
    pub OrOr = "||";
    /// `<<`
    pub Shl = "<<";
    /// `>>`
    pub Shr = ">>";
    /// `+=`
    pub PlusEq = "+=";
    /// `-=`
    pub MinusEq = "-=";
    /// `*=`
    pub StarEq = "*=";
    /// `/=`
    pub SlashEq = "/=";
    /// `%=`
    pub PercentEq = "%=";
    /// `^=`
    pub CaretEq = "^=";
    /// `&=`
    pub AndEq = "&=";
    /// `|=`
    pub OrEq = "|=";
    /// `<<=`
    pub ShlEq = "<<=";
    /// `>>=`
    pub ShrEq = ">>=";
    /// `=`
    pub Assign = "=";
    /// `==`
    pub Equal = "==";
    /// `!=`
    pub NotEqual = "!=";
    /// `>`
    pub Gt = ">";
    /// `<`
    pub Lt = "<";
    /// `>=`
    pub Ge = ">=";
    /// `<=`
    pub Le = "<=";
    /// `@`
    pub At = "@";
    /// `_`
    pub Underscore = "_";
    /// `.`
    pub Dot = ".";
    /// `..`
    pub DotDot = "..";
    /// `...`
    pub Ellipsis = "...";
    /// `..=`
    pub DotDotEq = "..=";
    /// `,`
    pub Comma = ",";
    /// `;`
    pub Semicolon = ";";
    /// `:`
    pub Colon = ":";
    /// `::`
    pub PathSep = "::";
    /// `->`
    pub RArrow = "->";
    /// `=>`
    pub FatArrow = "=>";
    /// `<-`
    pub LArrow = "<-";
    /// `#`
    pub Pound = "#";
    /// `$`
    pub Dollar = "$";
    /// `?`
    pub Question = "?";
    /// `~`
    pub Tilde = "~";
    /// `\`
    pub Backslash = "\\";
}
