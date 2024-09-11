//! This module contains types for punctuation tokens. These are used to represent single and
//! multi character punctuation tokens. For single character punctuation tokens, there are
//! there are `AnyPunct`, `AlonePunct` and `JointPunct` types.
//! Combined punctuation tokens are represented by `Operator`. The `operator!` macro can be
//! used to define custom operators.
#![allow(clippy::module_name_repetitions)]

pub use proc_macro2::Spacing;

use crate::{operator, Error, Parser, Punct, Result, ToTokens, TokenIter, TokenStream, TokenTree};

/// A single character punctuation token.
#[derive(Default, Clone)]
pub struct AnyPunct<const C: char>;

impl<const C: char> Parser for AnyPunct<C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == C => Ok(Self),
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl<const C: char> ToTokens for AnyPunct<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Punct::new(C, Spacing::Alone).to_tokens(tokens);
    }
}

/// Convert a `AnyPunct` object into a `TokenTree`.
impl<const C: char> From<AnyPunct<C>> for TokenTree {
    fn from(_: AnyPunct<C>) -> Self {
        TokenTree::Punct(Punct::new(C, Spacing::Alone))
    }
}

#[cfg(feature = "impl_display")]
impl<const C: char> std::fmt::Display for AnyPunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

#[cfg(feature = "impl_debug")]
impl<const C: char> std::fmt::Debug for AnyPunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyPunct<{C:?}>")
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
/// let colon = JointPunct::<':'>::parse(&mut token_iter).unwrap();
/// let colon = JointPunct::<':'>::parse(&mut token_iter).unwrap();
/// let colon = AnyPunct::<':'>::parse(&mut token_iter).unwrap();
///
/// // Caveat: The quote! macro won't join ':::' together
/// // let mut token_iter = quote::quote! {:::}.into_iter();
/// //
/// // let colon = JointPunct::<':'>::parse(&mut token_iter).unwrap();
/// // let colon = AnyPunct::<':'>::parse(&mut token_iter).unwrap();
/// // let colon = AnyPunct::<':'>::parse(&mut token_iter).unwrap();
/// ```
#[derive(Default, Clone)]
pub struct JointPunct<const C: char>;

impl<const C: char> JointPunct<C> {
    /// Create a new `JointPunct` object.
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

impl<const C: char> ToTokens for JointPunct<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Punct::new(C, Spacing::Joint).to_tokens(tokens);
    }
}

#[cfg(feature = "impl_display")]
impl<const C: char> std::fmt::Display for JointPunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

#[cfg(feature = "impl_debug")]
impl<const C: char> std::fmt::Debug for JointPunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JointPunct<{C:?}>")
    }
}

/// Convert a `JointPunct` object into a `TokenTree`.
impl<const C: char> From<JointPunct<C>> for TokenTree {
    fn from(_: JointPunct<C>) -> Self {
        TokenTree::Punct(Punct::new(C, Spacing::Joint))
    }
}

#[test]
fn test_joint_punct_into_tt() {
    let mut token_iter = "+=".to_token_iter();
    let plus = JointPunct::<'+'>::parser(&mut token_iter).unwrap();
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
/// let colon = AlonePunct::<':'>::parse(&mut token_iter).unwrap();
/// let colon = AlonePunct::<':'>::parse(&mut token_iter).unwrap();
/// ```
#[derive(Default, Clone)]
pub struct AlonePunct<const C: char>;

impl<const C: char> AlonePunct<C> {
    /// Create a new `AlonePunct` object.
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

impl<const C: char> Parser for AlonePunct<C> {
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

impl<const C: char> ToTokens for AlonePunct<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Punct::new(C, Spacing::Alone).to_tokens(tokens);
    }
}

#[cfg(feature = "impl_display")]
impl<const C: char> std::fmt::Display for AlonePunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{C}")
    }
}

#[cfg(feature = "impl_debug")]
impl<const C: char> std::fmt::Debug for AlonePunct<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlonePunct<{C:?}>")
    }
}

/// Convert a `AlonePunct` object into a `TokenTree`.
impl<const C: char> From<AlonePunct<C>> for TokenTree {
    fn from(_: AlonePunct<C>) -> Self {
        TokenTree::Punct(Punct::new(C, Spacing::Alone))
    }
}

#[test]
fn test_alone_punct_into_tt() {
    let mut token_iter = "+ +".to_token_iter();
    let plus = AlonePunct::<'+'>::parser(&mut token_iter).unwrap();
    assert_eq!(plus.as_char(), '+');
    let _: TokenTree = plus.into();
}

/// Operators made from up to 4 punctuation characters. Unused characters must be spaces.
/// Custom operators can be defined with the `operator!` macro. All but the last character are
/// `Spacing::Joint`. Attention must be payed when operators have the same prefix, the shorter
/// ones need to be tried first.
#[derive(Default, Clone)]
pub struct Operator<
    const C1: char,
    const C2: char = ' ',
    const C3: char = ' ',
    const C4: char = ' ',
>;

impl<const C1: char, const C2: char, const C3: char, const C4: char> Operator<C1, C2, C3, C4> {
    /// Create a new `Operator` object.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl<const C1: char, const C2: char, const C3: char, const C4: char> Parser
    for Operator<C1, C2, C3, C4>
{
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        if C2 == ' ' {
            AnyPunct::<C1>::parser(tokens)?;
            Ok(Self)
        } else {
            JointPunct::<C1>::parser(tokens)?;
            if C3 == ' ' {
                AnyPunct::<C2>::parser(tokens)?;
                Ok(Self)
            } else {
                JointPunct::<C2>::parser(tokens)?;
                if C4 == ' ' {
                    AnyPunct::<C3>::parser(tokens)?;
                    Ok(Self)
                } else {
                    JointPunct::<C3>::parser(tokens)?;
                    AnyPunct::<C4>::parser(tokens)?;
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
            if c == ' ' {
                Spacing::Alone
            } else {
                Spacing::Joint
            }
        }

        Punct::new(C1, spacing(C2)).to_tokens(tokens);
        if C2 != ' ' {
            Punct::new(C2, spacing(C3)).to_tokens(tokens);
            if C3 != ' ' {
                Punct::new(C3, spacing(C4)).to_tokens(tokens);
                if C4 != ' ' {
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
        if C4 != ' ' {
            write!(f, "{C1}{C2}{C3}{C4}")
        } else if C3 != ' ' {
            write!(f, "{C1}{C2}{C3}")
        } else if C2 != ' ' {
            write!(f, "{C1}{C2}")
        } else {
            write!(f, "{C1}")
        }
    }
}

#[cfg(feature = "impl_debug")]
impl<const C1: char, const C2: char, const C3: char, const C4: char> std::fmt::Debug
    for Operator<C1, C2, C3, C4>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if C4 != ' ' {
            write!(f, "Operator<'{C1}{C2}{C3}{C4}'>")
        } else if C3 != ' ' {
            write!(f, "Operator<'{C1}{C2}{C3}'>")
        } else if C2 != ' ' {
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
pub type LifetimeTick = JointPunct<'\''>;

operator! {
    /// `+`
    Plus = "+",
    /// `-`
    Minus = "-",
    /// `*`
    Star = "*",
    /// `/`
    Slash = "/",
    /// `%`
    Percent = "%",
    /// `^`
    Caret = "^",
    /// `!`
    Bang = "!",
    /// `&`
    And = "&",
    /// `|`
    Or = "|",
    /// `&&`
    AndAnd = "&&",
    /// `||`
    OrOr = "||",
    /// `<<`
    Shl = "<<",
    /// `>>`
    Shr = ">>",
    /// `+=`
    PlusEq = "+=",
    /// `-=`
    MinusEq = "-=",
    /// `*=`
    StarEq = "*=",
    /// `/=`
    SlashEq = "/=",
    /// `%=`
    PercentEq = "%=",
    /// `^=`
    CaretEq = "^=",
    /// `&=`
    AndEq = "&=",
    /// `|=`
    OrEq = "|=",
    /// `<<=`
    ShlEq = "<<=",
    /// `>>=`
    ShrEq = ">>=",
    /// `=`
    Assign = "=",
    /// `==`
    Equal = "==",
    /// `!=`
    NotEqual = "!=",
    /// `>`
    Gt = ">",
    /// `<`
    Lt = "<",
    /// `>=`
    Ge = ">=",
    /// `<=`
    Le = "<=",
    /// `@`
    At = "@",
    /// `_`
    Underscore = "_",
    /// `.`
    Dot = ".",
    /// `..`
    DotDot = "..",
    /// `...`
    Ellipsis = "...",
    /// `..=`
    DotDotEq = "..=",
    /// `,`
    Comma = ",",
    /// `;`
    Semicolon = ";",
    /// `:`
    Colon = ":",
    /// `::`
    PathSep = "::",
    /// `->`
    RArrow = "->",
    /// `=>`
    FatArrow = "=>",
    /// `<-`
    LArrow = "<-",
    /// `#`
    Pound = "#",
    /// `$`
    Dollar = "$",
    /// `?`
    Question = "?",
    /// `~`
    Tilde = "~",
    /// `\`
    Backslash = "\\",
}
