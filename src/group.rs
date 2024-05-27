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

impl Parse for ParenthesisGroup {
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

impl Parse for BraceGroup {
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

impl Parse for BracketGroup {
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

impl Parse for NoneGroup {
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
pub trait ParseGroup: Parse + sealed::Sealed {
    fn as_group(&self) -> &Group;
}

impl sealed::Sealed for Group {}
impl sealed::Sealed for ParenthesisGroup {}
impl sealed::Sealed for BraceGroup {}
impl sealed::Sealed for BracketGroup {}
impl sealed::Sealed for NoneGroup {}

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

/// A group and its contents
pub struct GroupContaining<G: ParseGroup, C: Parse> {
    pub group: G,
    pub content: C,
}

impl<G: ParseGroup, C: Parse> Parse for GroupContaining<G, C> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let group = G::parser(tokens)?;
        let mut c_iter = group.as_group().stream().into_iter();
        let content = C::parser(&mut c_iter)?;
        Ok(Self { group, content })
    }
}

/// `C` within `( )`
pub type ParenthesisGroupContaining<C> = GroupContaining<ParenthesisGroup, C>;
/// `C` within `{ }`
pub type BraceGroupContaining<C> = GroupContaining<BraceGroup, C>;
/// `C` within `[ ]`
pub type BracketGroupContaining<C> = GroupContaining<BracketGroup, C>;
/// `C` with no group delimiters
pub type NoneGroupContaining<C> = GroupContaining<NoneGroup, C>;
