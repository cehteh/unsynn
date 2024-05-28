use crate::*;

impl Parser for TokenTree {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(token) => Ok(token),
            None => Err("expected TokenTree, got end of stream".into()),
        }
    }
}

impl Parser for Group {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            Some(other) => Err(format!(
                "expected Group, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Group, got end of stream".into()),
        }
    }
}

impl Parser for Ident {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            Some(other) => Err(format!(
                "expected Ident, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Ident, got end of stream".into()),
        }
    }
}

impl Parser for Punct {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Punct(punct)) => Ok(punct),
            Some(other) => Err(format!(
                "expected Punct, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Punct, got end of stream".into()),
        }
    }
}

impl Parser for Literal {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(TokenTree::Literal(literal)) => Ok(literal),
            Some(other) => Err(format!(
                "expected Literal, got {:?} at {:?}",
                other,
                other.span().start()
            )
            .into()),
            None => Err("expected Literal, got end of stream".into()),
        }
    }
}

/// `TokenTree` with cached string representation.
pub type CachedTokenTree = Cached<TokenTree>;
/// `Group` with cached string representation.
pub type CachedGroup = Cached<Group>;
/// `Ident` with cached string representation.
pub type CachedIdent = Cached<Ident>;
/// `Punct` with cached string representation.
pub type CachedPunct = Cached<Punct>;
/// `Literal` with cached string representation.
pub type CachedLiteral = Cached<Literal>;
