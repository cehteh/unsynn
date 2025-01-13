//! This module contains macros and helper functions to define and parse custom types.
#[cfg(doc)]
use crate::*;

/// This macro supports the definition of enums, tuple structs and normal structs and
/// generates [`Parser`] and [`ToTokens`] implementations for them. It will implement `Debug`
/// and `Display`. Generics/Lifetimes are not supported (yet) on the primary type.
/// Note: eventually a derive macro for `Parser` and `ToTokens` will become supported by
/// a 'unsynn-derive' crate to give finer control over the expansion. `#[derive(Copy, Clone)]`
/// have to be manually defined. Keyword and operator definitions can also be defined,
/// they delegate to the `keyword!` and `operator!` macro described below. All entities can be
/// prefixed by `pub` to make them public.
///
/// Common for all variants is that entries are tried in order. Disjunctive for enums and
/// conjunctive in structures. This makes the order important, e.g. for enums, in case some
/// entries are subsets of others.
///
/// Enum variants without any data will never be parsed and will not generate any tokens.  For
/// *parsing* a enum that is optional one can add a variant like `None(Nothing)` at the end
/// (at the end is important, because Nothing always matches).
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// // Define some types
/// unsynn!{
///     keyword MyKeyword = "keyword";
///     // all items can be declared pub/pub(..) etc
///     pub(crate) operator MyOperator = "+++";
///
///     enum MyEnum {
///         /// Entries can have attributes/doc comments
///         Ident(Ident),
/// #       TupleWithDocs(
/// #           /// fooo
/// #           Ident,
/// #           /// bar
/// #           Optional<Ident>
/// #       ),
///         Braced(BraceGroup),
///         Text(LiteralString),
///         Number(LiteralInteger),
/// #       TrailingComma(LiteralInteger, Ident,),
///         Struct{
///             keyword: MyKeyword,
///             id: Ident,
///         },
///         // finally if nothing of the above matched, this will match.
///         None(Nothing),
///         // won't be parsed/matched at all
///         Empty,
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
///     // some enum variants
///     ident { within brace } "literal string" 1234 ()
///     // MyStruct fields
///     "literal string" 1234
///     // MyTupleStruct fields
///     ident "literal string"
///     // MyKeyword and MyOperator
///     keyword +++
/// "#.to_token_iter();
///
/// // Use the defined types
/// let MyEnum::Ident(myenum_ident) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_eq!(myenum_ident.tokens_to_string(), "ident");
/// let MyEnum::Braced(myenum_braced) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_eq!(myenum_braced.tokens_to_string(), "{within brace}".tokens_to_string());
/// let MyEnum::Text(myenum_text) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_eq!(myenum_text.tokens_to_string(), "\"literal string\"");
/// let MyEnum::Number(myenum_number) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_eq!(myenum_number.tokens_to_string(), "1234");
/// // the () will not be consumed by the MyEnum but match None(Nothing)
/// let myenum_nothing = MyEnum::parse(&mut token_iter).unwrap();
/// # assert_eq!(myenum_nothing.tokens_to_string(), "");
/// // consume the ()
/// <ParenthesisGroup>::parse(&mut token_iter).unwrap();
///
/// let my_struct =  MyStruct::parse(&mut token_iter).unwrap();
/// let my_tuple_struct =  MyTupleStruct::parse(&mut token_iter).unwrap();
/// let my_keyword =  MyKeyword::parse(&mut token_iter).unwrap();
/// let my_operator =  MyOperator::parse(&mut token_iter).unwrap();
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! unsynn {
    (enum $name:ident { $( $variant:ident... ),* }) => {};
    (struct $name:ident { $( $member:ident: $parser:ty ),* }) => {};
    (struct $name:ident ( $( $parser:ty ),*);) => {};
    (keyword $name:ident = "name";) => {};
    (operator $name:ident = "punct";) => {};
}

#[doc(hidden)]
#[cfg(not(doc))]
#[macro_export]
macro_rules! unsynn{
    // enums
    ($(#[$attribute:meta])* $pub:vis enum $name:ident {
        $($variants:tt)*
    } $($cont:tt)*) => {
        // The actual enum definition is written as given
        #[derive(Debug)]
        $(#[$attribute])* $pub enum $name {
            $($variants)*
        }

        impl Parser for $name {
            fn parser(tokens: &mut TokenIter) -> Result<Self> {
                let mut err = Error::no_error();
                // try to parse each variant
                $crate::unsynn!{@enum_parse_variant(tokens, err) $($variants)*}
                // nothing matched, error out
                Err(err)
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                $crate::unsynn!{@enum_to_tokens(self, tokens) {$($variants)*}}
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $crate::unsynn!{@enum_write(self, f) {$($variants)*}}
                Ok(())
            }
        }

        // next item
        $crate::unsynn!{$($cont)*}
    };

    // normal structs
    ($(#[$attribute:meta])* $pub:vis struct $name:ident {
        $($(#[$mattr:meta])* $mpub:vis $member:ident: $parser:ty),* $(,)?
    } $($cont:tt)*) => {
        #[derive(Debug)]
        $(#[$attribute])* $pub struct $name {
            $($(#[$mattr])* $mpub $member : $parser),*
        }

        impl $crate::Parser for $name {
            fn parser(tokens: &mut TokenIter) -> Result<Self> {
                Ok(Self{$($member: <$parser>::parser(tokens)?),*})
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                $(self.$member.to_tokens(tokens);)*
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $(write!(f, "{} ", &self.$member)?;)*
                Ok(())
            }
        }

        // next item
        $crate::unsynn!{$($cont)*}
    };

    // tuple structs
    ($(#[$attribute:meta])* $pub:vis struct $name:ident (
        $($(#[$mattr:meta])* $mpub:vis $parser:ty),* $(,)?
    ); $($cont:tt)*) => {
        #[derive(Debug)]
        $(#[$attribute])* $pub struct $name (
            $($(#[$mattr])* $mpub $parser),*
        );

        impl $crate::Parser for $name {
            fn parser(tokens: &mut TokenIter) -> Result<Self> {
                Ok(Self($(<$parser>::parser(tokens)?),*))
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                unsynn! {@tuple_for_each item in self : Self($($parser),*) {
                    item.to_tokens(tokens);
                }}
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                unsynn! {@tuple_for_each item in self : Self($($parser),*) {
                    write!(f, "{} " , &item)?;
                }}
                Ok(())
            }
        }

        // next item
        $crate::unsynn!{$($cont)*}
    };

    // keyword delegation
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident = $str:literal; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = $str}
        $crate::unsynn!{$($cont)*}
    };

    // operator delegation
    ($(#[$attribute:meta])* $pub:vis operator $name:ident = $str:literal; $($cont:tt)*) => {
        $crate::operator!{$(#[$attribute])* $pub $name = $str}
        $crate::unsynn!{$($cont)*}
    };

    // terminate recursion
    () => {};

    // to_tokens for enum tuple variant
    (@enum_to_tokens($self:ident, $tokens:ident) {$(#[$_attrs:meta])* $variant:ident($($tuple:tt)*) $(,$($cont:tt)*)?} ) => {
        if matches!($self, Self::$variant(..)) {
            unsynn! {@tuple_for_each item in $self : Self::$variant($($tuple)*) {
                item.to_tokens($tokens);
            }}
            return
        }
        $crate::unsynn!{@enum_to_tokens($self, $tokens) {$($($cont)*)?}}
    };

    // to_tokens for enum struct variant
    (@enum_to_tokens($self:ident, $tokens:ident) {
        $(#[$_attrs:meta])* $variant:ident {
            $($(#[$_mattrs:meta])* $member:ident: $_type:ty),* $(,)?
        } $(,$($cont:tt)*)?} ) => {
            if matches!($self, Self::$variant{..}) {
                $(
                    let Self::$variant{$member: member, ..} = $self else {unreachable!()};
                    member.to_tokens($tokens);
                )*
                return
            }
            $crate::unsynn!{@enum_to_tokens($self, $tokens) {$($($cont)*)?}}
    };

    // to_tokens for empty variant does nothing
    (@enum_to_tokens($self:ident, $tokens:ident) {$(#[$_attrs:meta])* $variant:ident $(,$($cont:tt)*)?} ) => {
        if matches!($self, Self::$variant) {
            return
        }
        $crate::unsynn!{@enum_to_tokens($self, $tokens) {$($($cont)*)?}}
    };

    // end recursion
    (@enum_to_tokens($self:ident, $tokens:ident) {}) => {};

    // write for enum tuple variant
    (@enum_write($self:ident, $f:ident) {$(#[$_attrs:meta])* $variant:ident($($tuple:tt)*) $(,$($cont:tt)*)?} ) => {
        if matches!($self, Self::$variant(..)) {
            unsynn! {@tuple_for_each item in $self : Self::$variant($($tuple)*) {
                write!($f, "{} " , &item)?;
            }}
        }
        $crate::unsynn!{@enum_write($self, $f) {$($($cont)*)?}}
    };

    // to_tokens for enum struct variant
    (@enum_write($self:ident, $f:ident) {
        $(#[$_attrs:meta])* $variant:ident {
            $($(#[$_mattrs:meta])* $member:ident: $_type:ty),* $(,)?
        } $(,$($cont:tt)*)?} ) => {
            if matches!($self, Self::$variant{..}) {
                $(
                    let Self::$variant{$member: that, ..} = $self else {unreachable!()};
                    write!($f, "{} ", that)?;
                )*
            }
            $crate::unsynn!{@enum_write($self, $f) {$($($cont)*)?}}
    };

    // write for empty variant does nothing
    (@enum_write($self:ident, $f:ident) {$(#[$_attrs:meta])* $variant:ident $(,$($cont:tt)*)?} ) => {
        if matches!($self, Self::$variant) {
        }
        $crate::unsynn!{@enum_write($self, $f) {$($($cont)*)?}}
    };

    // end recursion
    (@enum_write($self:ident, $f:ident) {}) => {};

    // Tuple enum variant
    (@enum_parse_variant($tokens:ident, $err:ident) $(#[$_attrs:meta])* $variant:ident($($tuple:tt)*) $(, $($cont:tt)*)?) => {
        if let Ok(parsed) = (|| -> $crate::Result<_> {
            $err.upgrade($crate::unsynn!{@enum_parse_tuple($tokens) $variant($($tuple)*)})
        })() {
            return Ok(parsed);
        }
        $crate::unsynn!{@enum_parse_variant($tokens, $err) $($($cont)*)?}
    };

    // Struct enum variant
    (@enum_parse_variant($tokens:ident, $err:ident) $(#[$_attrs:meta])* $variant:ident{$($members:tt)*} $(, $($cont:tt)*)?) => {
        if let Ok(parsed) = (|| -> $crate::Result<_> {
            $err.upgrade($crate::unsynn!{@enum_parse_struct($tokens) $variant{$($members)*}})
        })() {
            return Ok(parsed);
        }
        $crate::unsynn!{@enum_parse_variant($tokens, $err) $($($cont)*)?}
    };

    // Empty enum variant
    (@enum_parse_variant($tokens:ident, $err:ident) $(#[$_attrs:meta])* $variant:ident $(, $($cont:tt)*)?) => {
        /* NOP */
        $crate::unsynn!{@enum_parse_variant($tokens, $err) $($($cont)*)?}
    };

    // end recursion
    (@enum_parse_variant($tokens:ident, $err:ident)) => {};

    // Parse a tuple variant
    (@enum_parse_tuple($tokens:ident) $variant:ident($($(#[$_attrs:meta])* $parser:ty),* $(,)?)) => {
        $tokens.transaction(
            |mut tokens| Ok(Self::$variant($(<$parser>::parser(&mut tokens)?,)*))
        )
    };

    // Parse a struct variant
    (@enum_parse_struct($tokens:ident) $variant:ident{$($(#[$_attrs:meta])* $name:ident : $parser:ty),* $(,)?}) => {
        $tokens.transaction(
            |mut tokens| Ok(Self::$variant{$($name : <$parser>::parser(&mut tokens)?,)*})
        )
    };

    // iterate over $variant:($tuple) in $this and apply some $code for each $i
    (@tuple_for_each
        $i:ident in $this:ident :
        $($variant:ident)::*($($tuple:tt)*)
        {
            $($code:tt)*
        }
    ) => {
        {
            $crate::unsynn!{@tuple_for_each $i in $this : $($variant)::*[$($tuple)*] { $($code)* }}
        }
    };

    (@tuple_for_each
        $i:ident in $this:ident :
        $($variant:ident)::*[
            $(#[$_attrs:meta])* $_pub:vis $element:ty
            $(,$($rest:tt)*)?
        ]{
            $($code:tt)*
        }
    ) => {
        $crate::unsynn!{@tuple_for_each $i in $this : $($variant)::*[$($($rest)*)?] { $($code)* }}
        #[allow(irrefutable_let_patterns)]
        let $crate::unsynn!{@tuple_nth $i $($variant)::*[$($($rest)*)?]} = $this else {unreachable!()};
            $($code)*
        };
    (@tuple_for_each $i:ident in $_this:ident : $($variant:ident)::*[] { $($code:tt)* }) => {};

    // replaces each prefix item with a underscore, followed by $i and .. finally
    (@tuple_nth $i:ident $($variant:ident)::*[$($(#[$_attrs:meta])* $_pub:vis $_element:ty),* $(,)?]) => {
        $($variant)::*(
            $($crate::unsynn!(@_ $_element),)*
            $i,
            ..
        )
    };

    // replaces a single token with a underscore
    (@_ $_tt:tt) => {_}
}

/// Define types matching keywords.
///
/// `keyword!{ pub Name = "identifier", ...}`
///
/// * A optional `pub` defines the keyword public, default is private
/// * `Name` is the name for the struct to be generated
/// * `"identifier"` is the case sensitive keyword
///
/// `Name::parse()` will then only match the defined identifier.  It will implement `Debug`
/// and `Display` and `Clone` for keywords. Additionally `AsRef<str>` is implemented for each Keyword
/// to access the identifier string from rust code.
///
/// The `unsynn!` macro supports defining keywords by using `keyword Name = "ident";`, the
/// `pub` specification has to come before `keyword` then.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// keyword!{
///     /// Optional documentation for `If`
///     If = "if";
///     Else = "else";
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
    ($(#[$attribute:meta])* $pub:vis $name:ident = $str:literal $(;$($cont:tt)*)?) => {
        $(#[$attribute])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        $pub struct $name;

        impl $crate::Parser for $name {
            fn parser(tokens: &mut $crate::TokenIter) -> Result<Self> {
                use $crate::Parse;
                $crate::CachedIdent::parse_with(tokens, |ident, tokens| {
                    if ident == $str {
                        Ok($name)
                    } else {
                        $crate::Error::other::<$name>(
                            tokens,
                            format!(
                                "keyword {:?} expected, got {:?} at {:?}",
                                $str,
                                ident.as_str(),
                                ident.span().start()
                            )
                        )
                    }
                })
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                $crate::Ident::new($str, $crate::Span::call_site()).to_tokens(tokens);
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &$str
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} ", $str)
            }
        }
        $crate::keyword!{$($($cont)*)?}
    };
    () => {};
}

/// Define types matching operators (punctuation sequences).
///
/// `operator!{ pub Op = "punct"; ...}`
///
/// * A optional `pub` defines the operators public, default is private
/// * `Op` is the name for the struct to be generated
/// * `"punct"` is up to 4 ASCII punctuation characters
///
/// `Op::parse()` will match the defined operator. It will implement `Debug` and `Display`
/// and `Clone` for operators.
///
/// The `unsynn!` macro supports defining operators by using `operator Op = "chars";`, the
/// `pub` specification has to come before `operator` then.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// operator!{
///     /// Optional documentation for `<~~`
///     WLArrow = "<~~";
///     WRArrow = "~~>";
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
    // match a list of operator! defs
    ($($(#[$attribute:meta])* $pub:vis $name:ident = $op:literal);*$(;)?) => {
        $(
            $crate::operator!(@operator $(#[$attribute])* $pub $name = $op);
        )*
    };

    // match a single operator! defs with len 1-4
    (@operator $(#[$attribute:meta])* $pub:vis $name:ident = $op:literal) => {
        $(#[$attribute])*
        $pub type $name = Operator<
        {
                assert!(
                    $op.len() >= 1 && $op.len() <= 4,
                    "Operators must be 1-4 ASCII punctuation characters"
                );
                let c0 = $crate::operator!(@char_at 0 $op);
                assert!(c0.is_ascii_punctuation(), "Operator must be ASCII punctuation");
                c0
        },
        {
                let c1 = $crate::operator!(@char_at 1 $op);
                assert!(c1 == '\0' || c1.is_ascii_punctuation(), "Operator must be ASCII punctuation");
                c1
        },
        {
                let c2 = $crate::operator!(@char_at 2 $op);
                assert!(c2 == '\0' || c2.is_ascii_punctuation(), "Operator must be ASCII punctuation");
                c2
        },
        {
                let c3 = $crate::operator!(@char_at 3 $op);
                assert!(c3 == '\0' || c3.is_ascii_punctuation(), "Operator must be ASCII punctuation");
                c3
        },
        >;
    };

    // get a single ascii character from a literal string
    (@char_at $at:literal $op:literal) => {
       const {
           concat!($op, "\0\0\0").as_bytes()[$at] as char
       }
    }
}
