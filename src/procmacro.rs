use crate::*;

impl Parse for TokenTree {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(token) => Ok(token),
            None => Err("expected TokenTree, got end of stream".into()),
        }
    }
}

impl Parse for Group {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match ptokens.next() {
            Some(TokenTree::Group(group)) => {
                *tokens = ptokens;
                Ok(group)
            }
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

impl Parse for Ident {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match ptokens.next() {
            Some(TokenTree::Ident(ident)) => {
                *tokens = ptokens;
                Ok(ident)
            }
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

impl Parse for Punct {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match ptokens.next() {
            Some(TokenTree::Punct(punct)) => {
                *tokens = ptokens;
                Ok(punct)
            }
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

impl Parse for Literal {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        match ptokens.next() {
            Some(TokenTree::Literal(literal)) => {
                *tokens = ptokens;
                Ok(literal)
            }
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
