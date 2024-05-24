use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::*;

/// Zero or One of T
impl<T: Parse> Parse for Option<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match T::parse(tokens) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

/// Any number of T
impl<T: Parse> Parse for Vec<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut output = Vec::new();
        while let Ok(value) = T::parse(tokens) {
            output.push(value);
        }
        Ok(output)
    }
}

/// Box any parseable entity. In a enum it may happen that most variants are rather small
/// while few variants are large. In this case it may be beneficial to box the large variants.
impl<T: Parse> Parse for Box<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Box::new(T::parse(tokens)?))
    }
}

/// Rc any parseable entity. Just because we can. Sometimes when a value is shared between
/// multiple entities it may be beneficial to use Rc.
impl<T: Parse> Parse for Rc<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Rc::new(T::parse(tokens)?))
    }
}

/// Put any parseable entity in a RefCell. In case one wants to mutate the a parse tree on the
/// fly.
impl<T: Parse> Parse for RefCell<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        Ok(RefCell::new(T::parse(tokens)?))
    }
}

/// Put any parseable entity in a Cell. This is useful for when one has an immutable AST and
/// wants to swap values.
impl<T: Parse> Parse for Cell<T> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Cell::new(T::parse(tokens)?))
    }
}

/// Since the delimiter in `Delimited<T,D>` is optional a `Vec<Delimited<T,D>>` would parse
/// consecutive values even without delimiters. `DelimimitedVec<T,D>` will stop
/// parsing after the first value without a delimiter.
pub struct DelimitedVec<T: Parse, D: Parse>(pub Vec<Delimited<T, D>>);

impl<T: Parse, D: Parse> Parse for DelimitedVec<T, D> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
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

/// Vector of `T` delimited by `,`
pub type CommaDelimitedVec<T> = DelimitedVec<T, Comma>;
/// Vector of `T` delimited by `;`
pub type SemicolonDelimitedVec<T> = DelimitedVec<T, Semicolon>;
/// Vector of `T` delimited by `::'
pub type PathSepDelimitedVec<T> = DelimitedVec<T, PathSep>;
/// Vector of `T` delimited by `.`
pub type DotDelimitedVec<T> = DelimitedVec<T, Dot>;
/// Vector of `T` delimited by `:`
pub type ColonDelimitedVec<T> = DelimitedVec<T, Colon>;

/// Like `DelimitedVec` but with a minimum and maximum (inclusive) number of elements.
/// Parsing will succeed when the minimum number of elements is reached and stop at the
/// maximum number.  The delimiter `D` defaults to 'Nothing' to parse sequences which don't
/// have delimiters.
pub struct Repeats<const MIN: usize, const MAX: usize, T: Parse, D: Parse = Nothing>(
    pub Vec<Delimited<T, D>>,
);

impl<const MIN: usize, const MAX: usize, T: Parse, D: Parse> Parse for Repeats<MIN, MAX, T, D> {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut ptokens = tokens.clone();
        let mut output = Vec::new();
        while let Ok(value) = Delimited::<T, D>::parse(&mut ptokens) {
            let done = value.1.is_none();
            output.push(value);
            #[allow(unused_comparisons)]
            if done || output.len() >= MAX {
                break;
            }
        }

        #[allow(unused_comparisons)]
        if output.len() >= MIN {
            *tokens = ptokens;
            Ok(Self(output))
        } else {
            Err(format!(
                "expected Repeats<MIN={MIN}, MAX={MAX}, {}, {} >, got {:?} {:?} at {:?}",
                std::any::type_name::<T>(),
                std::any::type_name::<D>(),
                "foo",
                "bar",
                "baz"
            )
            .into())
        }
    }
}

/// Any number of T delimited by D or Nothing
pub type Any<T, D = Nothing> = Repeats<0, { usize::MAX }, T, D>;
/// One or more of T delimited by D or Nothing
pub type Many<T, D = Nothing> = Repeats<1, { usize::MAX }, T, D>;
/// Zero or one of T delimited by D or Nothing, i.e. optional (just for completeness)
pub type Optional<T, D = Nothing> = Repeats<0, 1, T, D>;
/// Exactly N of T delimited by D or Nothing
pub type Exactly<const N: usize, T, D = Nothing> = Repeats<N, N, T, D>;
/// At most N of T delimited by D or Nothing
pub type AtMost<const N: usize, T, D = Nothing> = Repeats<0, N, T, D>;
/// At least N of T delimited by D or Nothing
pub type AtLeast<const N: usize, T, D = Nothing> = Repeats<N, { usize::MAX }, T, D>;

// PLANNED: needs https://github.com/rust-lang/rust/issues/96097 impl<const N: usize, T: Parse> Parse for [T;N] {
