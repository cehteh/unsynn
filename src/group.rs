use crate::*;

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
            Some(other) => Err(format!(
                "expected ParenthesisGroup, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected ParenthesisGroup, got end of stream".into()),
        }
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
            Some(other) => Err(format!(
                "expected BraceGroup, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected BraceGroup, got end of stream".into()),
        }
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
            Some(other) => Err(format!(
                "expected BracketGroup, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected BracketGroup, got end of stream".into()),
        }
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
            Some(other) => Err(format!(
                "expected NoneGroup, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected NoneGroup, got end of stream".into()),
        }
    }
}

/// Common trait for all groups
pub trait ParserGroup: Parser + private::Sealed {
    /// Get the underlying group from any group type.
    fn as_group(&self) -> &Group;
}

impl private::Sealed for ParenthesisGroup {}
impl private::Sealed for BraceGroup {}
impl private::Sealed for BracketGroup {}
impl private::Sealed for NoneGroup {}
impl private::Sealed for Group {}

impl ParserGroup for Group {
    fn as_group(&self) -> &Group {
        self
    }
}

impl ParserGroup for ParenthesisGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

impl ParserGroup for BraceGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

impl ParserGroup for BracketGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

impl ParserGroup for NoneGroup {
    fn as_group(&self) -> &Group {
        &self.0
    }
}

/// Any kind of Group with some parseable content.
pub struct GroupContaining<G: ParserGroup, C: Parser> {
    /// The underlying group type. This can be `ParenthesisGroup`, `BraceGroup`,
    /// `BracketGroup`, `NoneGroup` or `Group`.
    pub group: G,
    /// The content of the group. That can be anything that implements `Parser`.
    pub content: C,
}

impl<G: ParserGroup, C: Parser> Parser for GroupContaining<G, C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let group = G::parser(tokens)?;
        let mut c_iter = group.as_group().stream().into_iter();
        let content = C::parser(&mut c_iter)?;
        Ok(Self { group, content })
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
