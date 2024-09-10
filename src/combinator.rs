//! A unique feature of unsynn is that one can define a parser as a composition of other !
//! parsers on the fly without the need to define custom structures. This is done by using the !
//! `Cons` and `Either` types. The [`Cons`] type is used to define a parser that is a !
//! conjunction of two to four other parsers, while the [`Either`] type is used to define a
//! parser that is a disjunction of two to four other parsers.

use crate::{Invalid, Nothing, Parse, Parser, Result, ToTokens, TokenIter, TokenStream};

/// Conjunctive `A` followed by `B` and optional `C` and `D`
/// When `C` and `D` are not used, they are set to `Nothing`.
#[derive(Clone)]
pub struct Cons<A, B, C = Nothing, D = Nothing> {
    /// The first value
    pub first: A,
    /// The second value
    pub second: B,
    /// The third value
    pub third: C,
    /// The fourth value
    pub fourth: D,
}

impl<A, B, C: 'static, D: 'static> Cons<A, B, C, D> {
    /// Return the number of consecutive used items (not `Nothing`) in a `Cons`
    ///
    /// # Panics
    ///
    /// Asserts that the `Cons` is not sparse, if `C` is not `Nothing` then `D` must not be
    /// `Nothing` either.
    // only used in Debug/Display
    #[allow(dead_code)]
    fn used_conjunctions() -> usize {
        let mut len = 2;
        // PLANNED: static NOTHING: TypeId once stable
        let nothing = std::any::TypeId::of::<Nothing>();
        if std::any::TypeId::of::<C>() != nothing {
            len += 1;
        }
        if std::any::TypeId::of::<D>() != nothing {
            assert_ne!(
                std::any::TypeId::of::<C>(),
                nothing,
                "If C is not Nothing then D must be Nothing"
            );
            len += 1;
        }
        len
    }
}

impl<A: Parse, B: Parse, C: Parse, D: Parse> Parser for Cons<A, B, C, D> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        Ok(Self {
            first: A::parser(tokens)?,
            second: B::parser(tokens)?,
            third: C::parser(tokens)?,
            fourth: D::parser(tokens)?,
        })
    }
}

impl<A: ToTokens, B: ToTokens, C: ToTokens, D: ToTokens> ToTokens for Cons<A, B, C, D> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.first.to_tokens(tokens);
        self.second.to_tokens(tokens);
        self.third.to_tokens(tokens);
        self.fourth.to_tokens(tokens);
    }
}

impl<A, B> From<Cons<A, B>> for (A, B) {
    fn from(cons: Cons<A, B>) -> Self {
        (cons.first, cons.second)
    }
}

impl<A, B, C> From<Cons<A, B, C>> for (A, B, C) {
    fn from(cons: Cons<A, B, C>) -> Self {
        (cons.first, cons.second, cons.third)
    }
}

impl<A, B, C, D> From<Cons<A, B, C, D>> for (A, B, C, D) {
    fn from(cons: Cons<A, B, C, D>) -> Self {
        (cons.first, cons.second, cons.third, cons.fourth)
    }
}

#[cfg(feature = "impl_debug")]
impl<A, B, C, D> std::fmt::Debug for Cons<A, B, C, D>
where
    A: std::fmt::Debug,
    B: std::fmt::Debug,
    C: std::fmt::Debug + 'static,
    D: std::fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match Self::used_conjunctions() {
            1 | 2 => f
                .debug_struct(&format!(
                    "Cons<{}, {}>",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>(),
                ))
                .field("first", &self.first)
                .field("second", &self.second)
                .finish(),
            3 => f
                .debug_struct(&format!(
                    "Cons<{}, {}, {}>",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>(),
                    std::any::type_name::<C>(),
                ))
                .field("first", &self.first)
                .field("second", &self.second)
                .field("third", &self.third)
                .finish(),
            _ => f
                .debug_struct(&format!(
                    "Cons<{}, {}, {}, {}>",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>(),
                    std::any::type_name::<C>(),
                    std::any::type_name::<D>(),
                ))
                .field("first", &self.first)
                .field("second", &self.second)
                .field("third", &self.third)
                .field("fourth", &self.fourth)
                .finish(),
        }
    }
}

#[cfg(feature = "impl_display")]
impl<A, B, C, D> std::fmt::Display for Cons<A, B, C, D>
where
    A: std::fmt::Display,
    B: std::fmt::Display,
    C: std::fmt::Display + 'static,
    D: std::fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match Self::used_conjunctions() {
            1 | 2 => write!(f, "{} {}", self.first, self.second),
            3 => write!(f, "{} {} {}", self.first, self.second, self.third),
            _ => write!(
                f,
                "{} {} {} {}",
                self.first, self.second, self.third, self.fourth
            ),
        }
    }
}

/// Disjunctive `A` or `B` tried in that order.
#[derive(Clone)]
pub enum Either<A, B> {
    /// The first alternative
    First(A),
    /// The second alternative
    Second(B),
}

impl<A, B> Either<A, B> {
    /// Deconstructs an `Either` and produces a common result type, independent of which
    /// alternative was present.
    pub fn fold<R, FF: FnOnce(A) -> R, SF: FnOnce(B) -> R>(self, first_fn: FF, second_fn: SF) -> R {
        match self {
            Either::First(a) => first_fn(a),
            Either::Second(b) => second_fn(b),
        }
    }

    /// Deconstructs an `Either` and produces a common result type for types that implement
    /// `Into<T>`.
    pub fn into<T>(self) -> T
    where
        A: Into<T>,
        B: Into<T>,
    {
        match self {
            Either::First(a) => a.into(),
            Either::Second(b) => b.into(),
        }
    }
}

impl<A: Parse, B: Parse> Parser for Either<A, B> {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        if let Ok(first) = A::parse(tokens) {
            Ok(Either::First(first))
        } else {
            Ok(Either::Second(B::parser(tokens)?))
        }
    }
}

impl<A: ToTokens, B: ToTokens> ToTokens for Either<A, B> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Either::First(a) => a.to_tokens(tokens),
            Either::Second(b) => b.to_tokens(tokens),
        }
    }
}

#[cfg(feature = "impl_debug")]
impl<A: std::fmt::Debug, B: std::fmt::Debug> std::fmt::Debug for Either<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Either::First(a) => f
                .debug_tuple(&format!(
                    "Either<{}, {}>::First",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>()
                ))
                .field(a)
                .finish(),
            Either::Second(b) => f.debug_tuple("Either::Second").field(b).finish(),
        }
    }
}

#[cfg(feature = "impl_display")]
impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Either<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Either::First(a) => write!(f, "{a}"),
            Either::Second(b) => write!(f, "{b}"),
        }
    }
}

#[test]
fn test_either_into_tt() {
    use crate::{LiteralInteger, TokenTree};
    let either = Either::<LiteralInteger, TokenTree>::First(LiteralInteger::new(42));
    let _tt: TokenTree = either.into();

    let either = Either::<TokenTree, LiteralInteger>::Second(LiteralInteger::new(43));
    let _tt: TokenTree = either.into();
}
