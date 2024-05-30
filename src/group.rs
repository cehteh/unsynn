#![allow(clippy::module_name_repetitions)]

use std::marker::PhantomData;

use crate::{
    private, Delimiter, EndOfStream, Error, Group, Parse, Parser, Result, ToTokens, TokenIter,
    TokenStream, TokenTree,
};

/// A group of tokens within `( )`
pub struct ParenthesisGroup(pub Group);

/// A group of tokens within `{ }`
pub struct BraceGroup(pub Group);

/// A group of tokens within `[ ]`
pub struct BracketGroup(pub Group);

/// A group of tokens with no delimiters
pub struct NoneGroup(pub Group);

impl From<ParenthesisGroup> for Group {
    fn from(group: ParenthesisGroup) -> Self {
        group.0
    }
}

impl Parser for ParenthesisGroup {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                Ok(Self(group))
            }
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for ParenthesisGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl From<BraceGroup> for Group {
    fn from(group: BraceGroup) -> Self {
        group.0
    }
}

impl Parser for BraceGroup {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                Ok(Self(group))
            }
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for BraceGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl From<BracketGroup> for Group {
    fn from(group: BracketGroup) -> Self {
        group.0
    }
}

impl Parser for BracketGroup {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                Ok(Self(group))
            }
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for BracketGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl From<NoneGroup> for Group {
    fn from(group: NoneGroup) -> Self {
        group.0
    }
}

impl Parser for NoneGroup {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => {
                Ok(Self(group))
            }
            Some(other) => Error::unexpected_token(other),
            None => Error::unexpected_end(),
        }
    }
}

impl ToTokens for NoneGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// Common trait for all groups
pub trait ParseGroup: Parse + private::Sealed {
    /// Get the underlying group from any group type.
    fn as_group(&self) -> &Group;
}

impl private::Sealed for ParenthesisGroup {}
impl private::Sealed for BraceGroup {}
impl private::Sealed for BracketGroup {}
impl private::Sealed for NoneGroup {}
impl private::Sealed for Group {}

impl ParseGroup for Group {
    fn as_group(&self) -> &Group {
        self
    }
}

impl ParseGroup for ParenthesisGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

impl ParseGroup for BraceGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

impl ParseGroup for BracketGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

impl ParseGroup for NoneGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

/// Any kind of Group `G` with parseable content `C`.  The content `C` must parse exhaustive,
/// a `EndOfStream` is automatically implied.
pub struct GroupContaining<G: ParseGroup, C: Parse> {
    /// The delimiters around the group.
    pub delimiter: Delimiter,
    /// The content of the group. That can be anything that implements `Parse`.
    pub content: C,
    group: PhantomData<G>,
}

impl<G: ParseGroup, C: Parse> Parser for GroupContaining<G, C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let group = G::parser(tokens)?;
        let mut c_iter = group.as_group().stream().into_iter();
        let content = C::parser(&mut c_iter)?;
        EndOfStream::parser(&mut c_iter)?;
        Ok(Self {
            delimiter: group.as_group().delimiter(),
            content,
            group: PhantomData,
        })
    }
}

impl<G: ParseGroup, C: Parse> ToTokens for GroupContaining<G, C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Group::new(self.delimiter, self.content.to_token_stream()).to_tokens(tokens);
    }
}

/// Parseable content within `( )`
pub type ParenthesisGroupContaining<C> = GroupContaining<ParenthesisGroup, C>;
/// Parseable content within `{ }`
pub type BraceGroupContaining<C> = GroupContaining<BraceGroup, C>;
/// Parseable content within `[ ]`
pub type BracketGroupContaining<C> = GroupContaining<BracketGroup, C>;
/// Parseable content with no group delimiters
pub type NoneGroupContaining<C> = GroupContaining<NoneGroup, C>;
