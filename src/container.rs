//! This module provides parsers for types that contain possibly multiple values.  This
//! includes `std` types like `Option`, `Vec`, `Box`, `Rc`, `RefCell` and types for delimited
//! and repeated values with numbered repeats.

use crate::{
    Colon, Comma, Delimited, Dot, Error, Nothing, Parse, Parser, PathSep, Result, Semicolon,
    ToTokens, TokenIter, TokenStream,
};

use std::{cell::RefCell, rc::Rc};

/// Zero or One of T
impl<T: Parse> Parser for Option<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        match T::parse(tokens) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

impl<T: Parse> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_some() {
            self.as_ref().unwrap().to_tokens(tokens);
        }
    }
}

/// Any number of T
impl<T: Parse> Parser for Vec<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut output = Vec::new();
        while let Ok(value) = T::parse(tokens) {
            output.push(value);
        }
        Ok(output)
    }
}

impl<T: Parse> ToTokens for Vec<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.iter().for_each(|value| value.to_tokens(tokens));
    }
}

/// Box any parseable entity. In a enum it may happen that most variants are rather small
/// while few variants are large. In this case it may be beneficial to box the large variants
/// to keep the enum lean.
impl<T: Parse> Parser for Box<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Box::new(T::parser(tokens)?))
    }
}

impl<T: Parse> ToTokens for Box<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_ref().to_tokens(tokens);
    }
}

/// Rc any parseable entity. Just because we can. Sometimes when a value is shared between
/// multiple entities it may be beneficial to use Rc.
impl<T: Parse> Parser for Rc<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Rc::new(T::parser(tokens)?))
    }
}

impl<T: Parse> ToTokens for Rc<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_ref().to_tokens(tokens);
    }
}

/// Put any parseable entity in a `RefCell`. In case one wants to mutate the a parse tree on the
/// fly.
impl<T: Parse> Parser for RefCell<T> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(RefCell::new(T::parser(tokens)?))
    }
}

impl<T: ToTokens> ToTokens for RefCell<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.borrow().to_tokens(tokens);
    }
}

/// Repeat trying to parse `S` and when this fails try to parse `T` and append that to a
/// vector, until a `S` is finally seen and stored. `S` may be a subset of `T`, thus parsing
/// become lazy and stopping at the first `S`.  This is the same as
/// `Cons<Vec<Cons<Except<S>,T>>>,S>` but more convenient and efficient.
///
/// # Example
///
/// Parse anything until a `;`.
///
/// ```
/// # use unsynn::*;
/// let mut token_iter = quote::quote! {foo bar ; baz ;}.into_iter();
///
/// type Example = LazyVec<TokenTree, Semicolon>;
///
/// let _example = Example::parse(&mut token_iter).unwrap();
/// let _example = Example::parse(&mut token_iter).unwrap();
/// ```
pub struct LazyVec<T: Parse, S: Parse>{
    /// The vector of repeating `T`
    pub vec: Vec<T>,
    /// The terminating `S`
    pub then: S,
}

impl<T: Parse, S: Parse> Parser for LazyVec<T, S> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut vec = Vec::new();

        loop {
            if let Ok(then) = S::parse(tokens) {
                return Ok(Self { vec, then });
            }

            vec.push(T::parse(tokens)?);
        }
    }
}

impl<T: Parse, S: Parse> ToTokens for LazyVec<T, S> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.vec.iter().for_each(|value| value.to_tokens(tokens));
        self.then.to_tokens(tokens);
    }
}

/// Since the delimiter in `Delimited<T,D>` is optional a `Vec<Delimited<T,D>>` would parse
/// consecutive values even without delimiters. `DelimimitedVec<T,D>` will stop
/// parsing after the first value without a delimiter.
pub struct DelimitedVec<T: Parse, D: Parse>(pub Vec<Delimited<T, D>>);

impl<T: Parse, D: Parse> Parser for DelimitedVec<T, D> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut output = Vec::new();
        while let Ok(value) = Delimited::<T, D>::parse(tokens) {
            let done = value.1.is_none();
            output.push(value);
            if done {
                break;
            }
        }
        Ok(Self(output))
    }
}

impl<T: Parse, D: Parse> ToTokens for DelimitedVec<T, D> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.iter().for_each(|value| value.to_tokens(tokens));
    }
}

/// Vector of `T` delimited by `,`
pub type CommaDelimitedVec<T> = DelimitedVec<T, Comma>;
/// Vector of `T` delimited by `;`
pub type SemicolonDelimitedVec<T> = DelimitedVec<T, Semicolon>;
/// Vector of `T` delimited by `::`
pub type PathSepDelimitedVec<T> = DelimitedVec<T, PathSep>;
/// Vector of `T` delimited by `.`
pub type DotDelimitedVec<T> = DelimitedVec<T, Dot>;
/// Vector of `T` delimited by `:`
pub type ColonDelimitedVec<T> = DelimitedVec<T, Colon>;

/// Like `DelimitedVec` but with a minimum and maximum (inclusive) number of elements.
/// Parsing will succeed when at least the minimum number of elements is reached and stop at
/// the maximum number.  The delimiter `D` defaults to 'Nothing' to parse sequences which
/// don't have delimiters.
pub struct Repeats<const MIN: usize, const MAX: usize, T: Parse, D: Parse = Nothing>(
    pub Vec<Delimited<T, D>>,
);

impl<const MIN: usize, const MAX: usize, T: Parse, D: Parse> Parser for Repeats<MIN, MAX, T, D> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        let mut output = Vec::new();
        while let Ok(value) = Delimited::<T, D>::parse(tokens) {
            let done = value.1.is_none();
            output.push(value);
            #[allow(unused_comparisons)]
            if done || output.len() >= MAX {
                break;
            }
        }

        #[allow(unused_comparisons)]
        if output.len() >= MIN {
            Ok(Self(output))
        } else {
            Error::other(format!(
                "less than MIN Repeats<MIN={MIN}, MAX={MAX}, {}, {} >, got {} repeats",
                std::any::type_name::<T>(),
                std::any::type_name::<D>(),
                output.len()
            ))
        }
    }
}

impl<const MIN: usize, const MAX: usize, T: Parse, D: Parse> ToTokens for Repeats<MIN, MAX, T, D> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.iter().for_each(|value| value.to_tokens(tokens));
    }
}

/// Any number of T delimited by D or Nothing
pub type Any<T, D = Nothing> = Repeats<0, { usize::MAX }, T, D>;
/// One or more of T delimited by D or Nothing
pub type Many<T, D = Nothing> = Repeats<1, { usize::MAX }, T, D>;
/// Zero or one of T delimited by D or Nothing
pub type Optional<T, D = Nothing> = Repeats<0, 1, T, D>;
/// Exactly N of T delimited by D or Nothing
pub type Exactly<const N: usize, T, D = Nothing> = Repeats<N, N, T, D>;
/// At most N of T delimited by D or Nothing
pub type AtMost<const N: usize, T, D = Nothing> = Repeats<0, N, T, D>;
/// At least N of T delimited by D or Nothing
pub type AtLeast<const N: usize, T, D = Nothing> = Repeats<N, { usize::MAX }, T, D>;

// PLANNED: needs https://github.com/rust-lang/rust/issues/96097 impl<const N: usize, T: Parse> Parser for [T;N] {
