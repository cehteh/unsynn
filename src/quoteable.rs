//! A wrapper type

#[allow(clippy::wildcard_imports)]
use crate::*;

/// Wraps a unsynn type so that it can be used within the `quote!{}` macro.
///
/// Unfortunately we can not use `impl quote:ToTokens for dyn unsynn::ToTokens`. The
/// `quote!{}` macro (or the current trait solver) won't pick that up. To mitigate this
/// problem we provide this `Quoteable<T>` type which implements `quote::ToTokens` and thus
/// can be used within the quote macro. The little inconvenience is that unsynn types have to
/// be wrapped in `Quoteable`.
///
/// This `Quoteable` wrapper type is always available, but the dependency and implementation
/// of `quote::ToTokens` is only available when the feature-flag `quote` is selected.
///
///
/// # Example
///
/// ```
/// # #[cfg(feature = "quote")] {
/// # use unsynn::*;
/// let quoteable = Quoteable::<Cons<ConstInteger<1>, Plus, ConstInteger<2>>>::default();
/// let quoted = quote! { let a = #quoteable;};
/// assert_eq!(quoted.tokens_to_string(), "let a = 1+2;".tokens_to_string());
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct Quoteable<T>(pub T);

impl<T> std::ops::Deref for Quoteable<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Parse> Parser for Quoteable<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self(T::parser(tokens).refine_err::<Self>()?))
    }
}

impl<T: ToTokens> ToTokens for Quoteable<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[cfg(feature = "quote")]
impl<T: ToTokens> quote::ToTokens for Quoteable<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        ToTokens::to_tokens(self, tokens);
    }
}
