use crate::*;
use std::fmt::Display;
use std::ops::Deref;

/// Getting the underlying source code as string from a parser is expensive. This struct
/// caches the string representation given entity.
///
/// # Example
///
/// ```
/// use unsynn::*;
/// let mut token_iter = quote::quote! {ident 1234}.into_iter();
///
/// let cached_ident = Cached::<Ident>::parse(&mut token_iter).unwrap();
/// assert!(cached_ident == "ident");
/// ```
pub struct Cached<T: Parser + ToString> {
    value: T,
    string: String,
}

impl<T: Parser + ToString> Parser for Cached<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let value = T::parser(tokens)?;
        let string = value.to_string();
        Ok(Self { value, string })
    }
}

impl<T: Parser + ToString> Cached<T> {
    /// Sets the value and updates the string representation.
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.string = self.value.to_string();
    }
}

impl<T: Parser + ToString> Deref for Cached<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Parser + ToString + Clone> Clone for Cached<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            string: self.string.clone(),
        }
    }
}

impl<T: Parser + ToString> PartialEq<&str> for Cached<T> {
    fn eq(&self, other: &&str) -> bool {
        &self.string == other
    }
}

impl<T: Parser + ToString> Display for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<T: Parser + ToString> AsRef<T> for Cached<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Parser + ToString> AsRef<str> for Cached<T> {
    fn as_ref(&self) -> &str {
        &self.string
    }
}
