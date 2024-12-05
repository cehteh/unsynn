//! Groups are a way to group tokens together. They are used to represent the contents between
//! `()`, `{}`, `[]` or no delimiters at all.  This module provides parser implementations for
//! opaque group types with defined delimiters and the [`GroupContaining`] types that parses the
//! surrounding delimiters and content of a group type.

#![allow(clippy::module_name_repetitions)]

use shadow_counted::IntoShadowCounted;

pub use proc_macro2::Delimiter;

use crate::{
    private, Cons, EndOfStream, Error, Group, Parse, Parser, Result, ToTokens, TokenIter,
    TokenStream, TokenTree,
};

macro_rules! make_group {
    ($($name:ident: $delimiter:ident);* $(;)?) => {
        $(
            #[doc = stringify!(A opaque group of tokens within a $delimiter)]
            #[cfg_attr(any(debug_assertions, feature = "impl_debug"), derive(Debug))]
            #[derive(Clone)]
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

            impl From<$name> for TokenTree {
                fn from(group: $name) -> Self {
                    group.0.into()
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

#[test]
fn test_bracegroup_into_tt() {
    let mut token_iter = "{a b c}".to_token_iter();
    let group = BraceGroup::parse(&mut token_iter).unwrap();
    let _: TokenTree = group.into();
}

/// Access to the surrounding `Delimiter` of a `GroupContaining` and its variants.
pub trait GroupDelimiter: private::Sealed {
    /// The surrounding `Delimiter` of the group.
    fn delimiter(&self) -> Delimiter;
}

/// Any kind of Group `G` with parseable content `C`.  The content `C` must parse exhaustive,
/// a [`EndOfStream`] is automatically implied.
#[derive(Clone)]
pub struct GroupContaining<C> {
    /// The delimiters around the group.
    pub delimiter: Delimiter,
    /// The content of the group.
    pub content: C,
}

impl<C> GroupContaining<C> {
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
    pub const fn new(delimiter: Delimiter, content: C) -> Self {
        Self { delimiter, content }
    }
}

impl<C: Parse> Parser for GroupContaining<C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let group = Group::parser(tokens)?;
        let mut c_iter = group.stream().into_iter().nested_shadow_counted(tokens);
        let content = C::parser(&mut c_iter)?;
        EndOfStream::parser(&mut c_iter)?;
        c_iter.commit();
        Ok(Self {
            delimiter: group.delimiter(),
            content,
        })
    }
}

impl<C: ToTokens> ToTokens for GroupContaining<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Group::new(self.delimiter, self.content.to_token_stream()).to_tokens(tokens);
    }
}

#[cfg(any(debug_assertions, feature = "impl_debug"))]
impl<C: std::fmt::Debug> std::fmt::Debug for GroupContaining<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(&format!("GroupContaining<{}>", std::any::type_name::<C>()))
            .field("delimiter", &self.delimiter)
            .field("content", &self.content)
            .finish()
    }
}

#[cfg(feature = "impl_display")]
impl<C: ToTokens> std::fmt::Display for GroupContaining<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}

impl<C> private::Sealed for GroupContaining<C> {}

impl<C> GroupDelimiter for GroupContaining<C> {
    fn delimiter(&self) -> Delimiter {
        self.delimiter
    }
}

impl<C: ToTokens> From<GroupContaining<C>> for TokenTree {
    fn from(group: GroupContaining<C>) -> Self {
        Group::new(group.delimiter(), group.content.to_token_stream()).into()
    }
}

#[test]
fn test_groupcontaining_into_tt() {
    let mut token_iter = "{a b c}".to_token_iter();
    let group = GroupContaining::<TokenStream>::parse(&mut token_iter).unwrap();
    let _: TokenTree = group.into();
}

macro_rules! make_group_containing {
    ($($name:ident: $delimiter:ident);* $(;)?) => {
        $(
            #[doc = stringify!(Parseable content within a $delimiter)]
            #[derive(Clone)]
            pub struct $name<C>{
                /// The inner content of the group.
                pub content: C
            }

            impl<C> $name<C> {
                #[doc = stringify!(create a new $name instance)]
                pub const fn new(content: C) -> Self {
                    Self{content}
                }
            }

            impl<C: Parse> Parser for $name<C> {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    match tokens.next() {
                        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::$delimiter => {
                            let mut counted = group
                                .stream()
                                .into_iter()
                                .nested_shadow_counted(tokens);

                            let content = Cons::<C, EndOfStream>::parser(&mut counted)?;
                            counted.commit();

                            Ok(Self{content: content.first})
                        }
                        Some(other) => Error::unexpected_token(other),
                        None => Error::unexpected_end(),
                    }
                }
            }

            impl<C: ToTokens> ToTokens for $name<C> {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    Group::new(Delimiter::$delimiter, self.content.to_token_stream()).to_tokens(tokens);
                }
            }

            #[cfg(any(debug_assertions, feature = "impl_debug"))]
            impl<C: std::fmt::Debug> std::fmt::Debug
                for $name<C>
            {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.debug_tuple(&format!(
                        stringify!($name<{}>),
                        std::any::type_name::<C>()
                    ))
                     .field(&self.content)
                     .finish()
                }
            }

            #[cfg(feature = "impl_display")]
            impl<C: ToTokens> std::fmt::Display for $name<C> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.to_token_stream())
                }
            }

            impl<C> private::Sealed for $name<C> {}

            impl<C> GroupDelimiter for $name<C> {
                fn delimiter(&self) -> Delimiter {
                    Delimiter::$delimiter
                }
            }

            impl<C: ToTokens> From<$name<C>> for TokenTree {
                fn from(group: $name<C>) -> Self {
                    Group::new(Delimiter::$delimiter, group.content.to_token_stream()).into()
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

#[test]
fn test_bracegroupcontaining_into_tt() {
    let mut token_iter = "{a b c}".to_token_iter();
    let group = BraceGroupContaining::<TokenStream>::parse(&mut token_iter).unwrap();
    let _: TokenTree = group.into();
}
