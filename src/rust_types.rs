//! Parsers for rusts types.

use crate::{
    Error, Ident, LiteralCharacter, LiteralInteger, Parse, Parser, Result, Span, ToTokens,
    TokenIter, TokenStream,
};

// Parser and ToTokens for unsigned integer types
macro_rules! impl_unsigned_integer {
    ($($ty:ty),*) => {
        $(
            #[doc = stringify!(Parse $ty may have a positive sign but no suffix)]
            impl Parser for $ty {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    let lit = crate::Cons::<Option<crate::Plus>, LiteralInteger>::parser(tokens)?;
                    <$ty>::try_from(lit.second.value()).map_err(Error::boxed)
                }
            }

            #[doc = stringify!(Emit a literal $ty without sign and suffix)]
            impl ToTokens for $ty {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    LiteralInteger::new(*self as u128).to_tokens(tokens);
                }
            }
        )*
    };
}

impl_unsigned_integer! {u8, u16, u32, u64, u128, usize}

// Parser and ToTokens for signed integer types
macro_rules! impl_signed_integer {
    ($($ty:ty),*) => {
        $(
            #[doc = stringify!(Parse $ty may have a positive or negative sign but no suffix)]
            impl Parser for $ty {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    let lit = crate::Cons::<Option<crate::Either<crate::Plus, crate::Minus>>, LiteralInteger>::parser(tokens)?;
                    <$ty>::try_from(lit.second.value())
                    .map_err(Error::boxed)
                    .and_then(|value| {
                        match lit.first {
                            Some(crate::Either::Second(_)) => Ok(-value),
                            _ => Ok(value),
                        }
                    })
                }
            }

            #[doc = stringify!(Emit a literal $ty with negative sign and without suffix)]
            impl ToTokens for $ty {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    if *self < 0 {
                        crate::Minus::new().to_tokens(tokens);
                    }
                    LiteralInteger::new(self.abs().try_into().unwrap()).to_tokens(tokens);
                }
            }
        )*
    };
}

impl_signed_integer! {i8, i16, i32, i64, i128, isize}

// Parser and ToTokens for char
impl Parser for char {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let lit = LiteralCharacter::parser(tokens)?;
        Ok(lit.value())
    }
}

impl ToTokens for char {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        LiteralCharacter::new(*self).to_tokens(tokens);
    }
}

// Parser and ToTokens for bool
/// Parse a boolean value from the input stream.
/// Only `true` and `false` are valid boolean values.
impl Parser for bool {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ident::parse_with(tokens, |ident| {
            if ident == "true" {
                Ok(true)
            } else if ident == "false" {
                Ok(false)
            } else {
                Error::unexpected_token(ident.into())
            }
        })
    }
}

impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Ident::new(if *self { "true" } else { "false" }, Span::call_site()).to_tokens(tokens);
    }
}
