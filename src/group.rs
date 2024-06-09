//! Groups are a way to group tokens together. They are used to represent the contents between
//! `()`, `{}`, `[]` or no delimiters at all.  This module provides parser implementations for
//! opaque group types with defined delimiters and the [`GroupContaining`] types that parses the
//! surrounding delimiters and content of a group type.

#![allow(clippy::module_name_repetitions)]

pub use proc_macro2::Delimiter;

use crate::{
    private, Cons, EndOfStream, Error, Group, Parse, Parser, Result, ToTokens, TokenIter,
    TokenStream, TokenTree,
};

macro_rules! make_group {
    ($($name:ident: $delimiter:ident);* $(;)?) => {
        $(
            /// A opaque group of tokens within $delimiter
            #[cfg_attr(feature = "impl_debug", derive(Debug))]
            pub struct $name(pub Group);

            impl From<$name> for Group {
                fn from(group: $name) -> Self {
                    group.0
                }
            }

            impl Parser for $name {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    match tokens.next() {
                        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::$delimiter => {
                            Ok(Self(group))
                        }
                        Some(other) => Error::unexpected_token(other),
                        None => Error::unexpected_end(),
                    }
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    self.0.to_tokens(tokens);
                }
            }

            impl private::Sealed for $name {}

            impl GroupDelimiter for $name {
                fn delimiter(&self) -> Delimiter {
                    Delimiter::$delimiter
                }
            }

            #[cfg(feature = "impl_display")]
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        )*
    };
}

make_group! {
    ParenthesisGroup: Parenthesis;
    BraceGroup: Brace;
    BracketGroup: Bracket;
    NoneGroup: None;
}

/// Access to the surrounding `Delimiter` of a `GroupContaining` and its variants.
pub trait GroupDelimiter: private::Sealed {
    /// The surrounding `Delimiter` of the group.
    fn delimiter(&self) -> Delimiter;
}

/// Access to the content of a `GroupContaining` and its variants.
pub trait GroupContent<C: Parse>: private::Sealed {
    /// The content of the group.
    fn content(&self) -> &C;
}

/// Any kind of Group `G` with parseable content `C`.  The content `C` must parse exhaustive,
/// a `EndOfStream` is automatically implied.
pub struct GroupContaining<C: Parse> {
    /// The delimiters around the group.
    pub delimiter: Delimiter,
    /// The content of the group. That can be anything that implements `Parse`.
    pub content: C,
}

impl<C: Parse> GroupContaining<C> {
    /// Create a new `GroupContaining` instance.
    ///
    /// # Example
    ///
    /// ```
    /// # use unsynn::*;
    ///
    /// let group = GroupContaining::new(
    ///     Delimiter::Parenthesis,
    ///     Literal::i32_unsuffixed(123),
    /// );
    /// # #[cfg(feature = "impl_display")]
    /// # assert_eq!(group.to_string(), "(123)");
    /// ```
    pub fn new(delimiter: Delimiter, content: C) -> Self {
        Self { delimiter, content }
    }
}

impl<C: Parse> Parser for GroupContaining<C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let group = Group::parser(tokens)?;
        let mut c_iter = group.stream().into_iter();
        let content = C::parser(&mut c_iter)?;
        EndOfStream::parser(&mut c_iter)?;
        Ok(Self {
            delimiter: group.delimiter(),
            content,
        })
    }
}

impl<C: Parse> ToTokens for GroupContaining<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Group::new(self.delimiter, self.content.to_token_stream()).to_tokens(tokens);
    }
}

#[cfg(feature = "impl_debug")]
impl<C: Parse + std::fmt::Debug> std::fmt::Debug for GroupContaining<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(&format!("GroupContaining<{}>", std::any::type_name::<C>()))
            .field("delimiter", &self.delimiter)
            .field("content", &self.content)
            .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<C: Parse> std::fmt::Display for GroupContaining<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}

impl<C: Parse> private::Sealed for GroupContaining<C> {}

impl<C: Parse> GroupDelimiter for GroupContaining<C> {
    fn delimiter(&self) -> Delimiter {
        self.delimiter
    }
}

impl<C: Parse> GroupContent<C> for GroupContaining<C> {
    fn content(&self) -> &C {
        &self.content
    }
}

macro_rules! make_group_containing {
    ($($name:ident: $delimiter:ident);* $(;)?) => {
        $(
            /// Parseable content within `$delimiter`
            pub struct $name<C: Parse>(C);

            impl<C: Parse> $name<C> {
                /// Create a new `$name` instance.
                pub fn new(content: C) -> Self {
                    Self(content)
                }
            }

            impl<C: Parse> Parser for $name<C> {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    match tokens.next() {
                        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::$delimiter => {
                            Ok(Self(Cons::<C, EndOfStream>::parser(&mut group.stream().into_iter())?.0))
                        }
                        Some(other) => Error::unexpected_token(other),
                        None => Error::unexpected_end(),
                    }
                }
            }

            impl<C: Parse> ToTokens for $name<C> {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    Group::new(Delimiter::$delimiter, self.0.to_token_stream()).to_tokens(tokens);
                }
            }

            #[cfg(feature = "impl_debug")]
            impl<C: Parse + std::fmt::Debug> std::fmt::Debug
                for $name<C>
            {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.debug_tuple(&format!(
                        stringify!($name<{}>),
                        std::any::type_name::<C>()
                    ))
                     .field(&self.0)
                     .finish()
                }
            }

            #[cfg(feature = "impl_display")]
            impl<C: Parse> std::fmt::Display for $name<C> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.to_token_stream())
                }
            }

            impl<C: Parse> private::Sealed for $name<C> {}

            impl<C: Parse> GroupDelimiter for $name<C> {
                fn delimiter(&self) -> Delimiter {
                    Delimiter::$delimiter
                }
            }

            impl<C: Parse> GroupContent<C> for $name<C> {
                fn content(&self) -> &C {
                    &self.0
                }
            }
        )*
    };
}

make_group_containing! {
    ParenthesisGroupContaining: Parenthesis;
    BraceGroupContaining: Brace;
    BracketGroupContaining: Bracket;
    NoneGroupContaining: None;
}
