#[cfg(doc)]
use crate::*;

/// This macro supports the definition of enums, tuple structs and normal structs and
/// generates [`Parser`] and [`ToTokens`] implementations for them. It will implement `Debug`
/// and `Display` if the `impl_debug` and `impl_display` features are
/// enabled. Generics/Lifetimes are not supported (yet). Note: eventually a derive macro for
/// `Parser` and `ToTokens` will become supported by a 'unsynn-derive' crate to give finer
/// control over the expansion. `#[derive(Copy, Clone)]` have to be manually defined. `Debug`
/// and `Display` are automatically implemented when the respective features are enabled.
///
/// Common for all three variants is that entries are tried in order. Disjunctive for enums
/// and conjunctive in structures. This makes the order important, e.g. for enums, in case
/// some entries are subsets of others.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// // Define some types
/// unsynn!{
///     enum MyEnum {
///         Ident(Ident),
///         Braced(BraceGroup),
///         Text(LiteralString),
///         Number(LiteralInteger),
///     }
///
///     struct MyStruct {
///         text: LiteralString,
///         number: LiteralInteger,
///     }
///
///     struct MyTupleStruct(Ident, LiteralString);
/// }
///
/// // Create an iterator over the things we want to parse
/// let mut token_iter = r#"
///     ident { within brace } "literal string" 1234
///     "literal string" 1234
///     ident "literal string"
/// "#.to_token_iter();
///
/// // Use the defined types
/// let MyEnum::Ident(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// let MyEnum::Braced(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// let MyEnum::Text(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// let MyEnum::Number(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
///
/// let my_struct =  MyStruct::parser(&mut token_iter).unwrap();
///
/// let my_tuple_struct =  MyTupleStruct::parser(&mut token_iter).unwrap();
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! unsynn {
    (enum $name:ident { $( $variant:ident($parser:ty) ),* }) => {};
    (struct $name:ident { $( $member:ident: $parser:ty ),* }) => {};
    (struct $name:ident ( $( $parser:ty ),*);) => {};
}

#[doc(hidden)]
#[cfg(not(doc))]
#[macro_export]
macro_rules! unsynn{
    ($(#[$attribute:meta])* $pub:vis enum $name:ident {
        $($(#[$vattr:meta])* $variant:ident($parser:ty)),* $(,)?
    } $($cont:tt)*) => {
        #[cfg_attr(feature = "impl_debug", derive(Debug))]
        $(#[$attribute])* $pub enum $name {
            $($(#[$vattr])* $variant($parser)),*
        }

        impl Parser for $name {
            fn parser(tokens: &mut TokenIter) -> Result<Self> {
                $(
                    if let Ok(parsed) = <$parser>::parse(tokens) {
                        return Ok($name::$variant(parsed));
                    }
                )*
                    match tokens.next() {
                        Some(token) => $crate::Error::unexpected_token(token),
                        None => $crate::Error::unexpected_end()
                    }
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(
                        $name::$variant(matched) => matched.to_tokens(tokens),
                    )*
                }
            }
        }

        #[cfg(feature = "impl_display")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $name::$variant(matched) => write!(f, "{matched} "),
                    )*
                }
            }
        }
        $crate::unsynn!{$($cont)*}
    };

    ($(#[$attribute:meta])* $pub:vis struct $name:ident {
        $($(#[$mattr:meta])* $mpub:vis $member:ident: $parser:ty),* $(,)?
    } $($cont:tt)*) => {
        #[cfg_attr(feature = "impl_debug", derive(Debug))]
        $(#[$attribute])* $pub struct $name {
            $($(#[$mattr])* $mpub $member : $parser),*
        }

        impl Parser for $name {
            fn parser(tokens: &mut TokenIter) -> Result<Self> {
                Ok(Self{$($member: <$parser>::parser(tokens)?),*})
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                $(self.$member.to_tokens(tokens);)*
            }
        }

        #[cfg(feature = "impl_display")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $(write!(f, "{} ", &self.$member);)*
                    Ok(())
            }
        }
        $crate::unsynn!{$($cont)*}
    };

    ($(#[$attribute:meta])* $pub:vis struct $name:ident (
        $($(#[$mattr:meta])* $mpub:vis $parser:ty),* $(,)?
    ); $($cont:tt)*) => {
        #[cfg_attr(feature = "impl_debug", derive(Debug))]
        $(#[$attribute])* $pub struct $name (
            $($(#[$mattr])* $mpub $parser),*
        );

        impl Parser for $name {
            fn parser(tokens: &mut TokenIter) -> Result<Self> {
                Ok(Self($(<$parser>::parser(tokens)?),*))
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                $crate::unsynn!{@tuple_to_tokens $name(self, tokens) $($parser),*}
            }
        }

        #[cfg(feature = "impl_display")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $crate::unsynn!{@tuple_write $name(self, f) $($parser),*}
                Ok(())
            }
        }
        $crate::unsynn!{$($cont)*}
    };

    // terminate recursion
    () => {};

    // For the tuple struct ToTokens impl we need to match each tuple member and call to_tokens on it
    (@tuple_to_tokens $name:ident($this:ident,$param:ident) $element:ty $(,$rest:ty)* $(,)?) => {
        $crate::unsynn!{@tuple_to_tokens $name($this,$param) $($rest),*}
        let $name($($crate::unsynn!{@_ $rest},)*  that, .. ) = $this;
        that.to_tokens($param);
    };
    (@tuple_to_tokens $name:ident($this:ident,$param:ident)) => {};

    // same for write
    (@tuple_write $name:ident($this:ident,$f:ident) $element:ty $(,$rest:ty)* $(,)?) => {
        $crate::unsynn!{@tuple_write $name($this,$f) $($rest),*}
        let $name($($crate::unsynn!{@_ $rest},)*  that, .. ) = $this;
        write!($f, "{} ", &that)?;
    };
    (@tuple_write $name:ident($this:ident,$f:ident)) => {};

    // replaces a single token with a underscore
    (@_ $unused:tt) => {_};
}

/// Define types matching keywords.
///
/// `keyword!{ Name = "identifier", ...}`
///
/// * `Name` is the name for the struct to be generated
/// * `"identifier"` is the case sensitive keyword
/// * keywords are always defined as `pub`
///
/// `Name::parse()` will then only match the defined identifier.  It will implement `Debug`
/// and `Display` if the `impl_debug` and `impl_display` features are enabled. `Clone` is
/// always implemented for keywords. Additionally `AsRef<str>` is implemented for each Keyword
/// to access the identifier string from rust code.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// keyword!{
///     /// Optional documentation for `If`
///     If = "if",
///     Else = "else",
/// }
///
/// let mut tokens = "if else".to_token_iter();
/// let if_kw = If::parse(&mut tokens).unwrap();
/// assert_eq!(if_kw.as_ref(), "if");
/// let else_kw = Else::parse(&mut tokens).unwrap();
/// assert_eq!(else_kw.as_ref(), "else");
/// ```
#[macro_export]
macro_rules! keyword{
    ($($(#[$attribute:meta])* $name:ident = $str:literal),*$(,)?) => {
        $(
            $(#[$attribute])*
            #[cfg_attr(feature = "impl_debug", derive(Debug))]
            #[derive(Clone)]
            pub struct $name;

            impl Parser for $name {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    use $crate::Parse;
                    $crate::CachedIdent::parse_with(tokens, |ident| {
                        if ident == $str {
                            Ok($name)
                        } else {
                            $crate::Error::other::<$name>(
                                format!(
                                    "keyword {:?} expected, got {:?} at {:?}",
                                    $str,
                                    ident.string(),
                                    ident.span().start()
                                )
                            )
                        }
                    })
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    $crate::Ident::new($str, $crate::Span::call_site()).to_tokens(tokens);
                }
            }

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    &$str
                }
            }

            #[cfg(feature = "impl_display")]
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} ", $str)
                }
            }
        )*
    }
}

/// Define types matching operators (punctuation sequences).
///
/// `operator!{ Op = "punct", ...}`
///
/// * `Op` is the name for the struct to be generated
/// * `"punct"` is up to 4 punctuation characters
/// *  The validity of the characters is **not** checked
/// * operators are always defined as `pub`
///
/// `Op::parse()` will match the defined operator. It will implement `Debug` and `Display` if
/// the `impl_debug` and `impl_display` features are enabled. `Clone` is always implemented
/// for operators.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// operator!{
///     /// Optional documentation for `<~~`
///     WLArrow = "<~~",
///     WRArrow = "~~>",
/// }
///
/// let mut tokens = "<~~~~> ~~><~~".to_token_iter();
/// let wl = WLArrow::parse(&mut tokens).unwrap();
/// assert_eq!(wl.tokens_to_string(), "<~~");
/// let wr = WRArrow::parse(&mut tokens).unwrap();
/// assert_eq!(wr.tokens_to_string(), "~~>");
/// # let wr = WRArrow::parse(&mut tokens).unwrap();
/// # assert_eq!(wr.tokens_to_string(), "~~>");
/// # let wl = WLArrow::parse(&mut tokens).unwrap();
/// # assert_eq!(wl.tokens_to_string(), "<~~");
/// ```
#[macro_export]
macro_rules! operator{
    // match a list punct! defs
    ($($(#[$attribute:meta])* $name:ident = $op:literal),*$(,)?) => {
        $(
            $crate::operator!(@operator $(#[$attribute])* $name = $op);
        )*
    };

    // match a single punct! defs with len 1-3
    (@operator $(#[$attribute:meta])* $name:ident = $op:literal) => {
        $(#[$attribute])*
        pub type $name = Operator<
        {$crate::operator!(@char_at 0 $op)},
        {$crate::operator!(@char_at 1 $op)},
        {$crate::operator!(@char_at 2 $op)},
        {$crate::operator!(@char_at 3 $op)},
        >;
    };

    // get a single ascii character from a literal string
    (@char_at $at:literal $op:literal) => {
       const {
           concat!($op, "   ").as_bytes()[$at] as char
       }
    }
}
