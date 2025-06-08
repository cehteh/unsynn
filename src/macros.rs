//! This module contains the big macros and helper functions to define and parse custom types.

#[cfg(doc)]
use crate::*;

/// This macro supports the definition of enums, tuple structs and normal structs and
/// generates [`Parser`] and [`ToTokens`] implementations for them. It will derive `Debug`.
/// Generics/Lifetimes are not supported on the primary type.  Note: eventually a derive macro
/// for `Parser` and `ToTokens` will become supported by a 'unsynn-derive' crate to give finer
/// control over the expansion. `#[derive(Copy, Clone)]` have to be manually defined. Keyword
/// and operator definitions can also be defined, they delegate to the `keyword!` and
/// `operator!` macro described below. All entities can be prefixed by `pub` to make them
/// public. Type aliases, function definitions, macros and use statements are passed through. This
/// makes thing easier readable when you define larger unsynn macro blocks.
///
/// The macro definition above is simplified for readability `struct`, `enum` and `type`
/// definitions can include most of the things normal rust definitions can do. This also
/// includes definitions of members of structs and enums:
///
/// * Any number of attributes (`#[...]`), including documentation comments. Note that the
///   unsynn macros have limited support for automatically generation documentation. This
///   auto-generated documentation is appended after the user supplied docs.
/// * structs, enums, types and members can exported with the usual `pub` declarations.
/// * struct, enum, and type definitions support generics. These generics can include simple
///   trait bounds and defaults. The traits for the bounds have to be in scope since for
///   simplicity only single identifiers are allowed. These simple trait bounds can be defined
///   with where-clauses as well. Lifetimes and HRTB are not supported.
///
/// Common for enum and struct variants is that entries are tried in order. Disjunctive for
/// enums and conjunctive in structures. This makes the order important, e.g. for enums, in
/// case some entries are subsets of others.
///
/// Enum variants without any data will never be parsed and will not generate any tokens. For
/// *parsing* a enum that is optional one can add a variant like `None(Nothing)` at the end
/// (at the end is important, because Nothing always matches).
///
///
/// ## Example
///
/// ```
/// # use unsynn::*;
/// // must be in scope to be used as constraint
/// use std::fmt::Debug;
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
///     // With generics and type defaults
///     struct MyStruct<T: Debug = i32> {
///         text: LiteralString,
///         number: T,
///     }
///
///     struct MyTupleStruct(Ident, LiteralString);
///
///     // type definitions are pass-through.
///     pub type Alias = MyStruct<LiteralInteger>;
///
///     // functions are pass though too
///     fn testfn() -> bool { true }
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
/// # assert_tokens_eq!(myenum_ident, "ident");
/// let MyEnum::Braced(myenum_braced) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_tokens_eq!(myenum_braced, "{within brace}");
/// let MyEnum::Text(myenum_text) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_tokens_eq!(myenum_text, "\"literal string\"");
/// let MyEnum::Number(myenum_number) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// # assert_tokens_eq!(myenum_number, "1234");
/// // the () will not be consumed by the MyEnum but match None(Nothing)
/// let myenum_nothing = MyEnum::parse(&mut token_iter).unwrap();
/// # assert_tokens_eq!(myenum_nothing, "");
/// // consume the ()
/// <ParenthesisGroup>::parse(&mut token_iter).unwrap();
///
/// let my_struct =  Alias::parse(&mut token_iter).unwrap();
/// let my_tuple_struct =  MyTupleStruct::parse(&mut token_iter).unwrap();
/// let my_keyword =  MyKeyword::parse(&mut token_iter).unwrap();
/// let my_operator =  MyOperator::parse(&mut token_iter).unwrap();
/// # assert!(testfn());
/// ```
///
///
/// # Traits within the `unsynn!` macro ***(EXPERIMENTAL)***
///
/// The `unsynn!` macro has limited but ergonomic support for defining and implementing
/// traits. The main purpose for this is to be able to define simple markers and accessors
/// this helps with compile-time validation of grammars.
///
/// Trait definitions are passed though. There are simplifications that they are allowed to
/// end with a `;` instead a `{}`. This signifies simple marker traits that have no
/// methods. The form `trait {Name; ...}` is supported to define a set of simple markers
/// in one go. This syntax not support traits with methods.
///
/// Trait implementations are either passed through or are a `impl {}` block directly
/// following an ADT definition. In the later case most boilerplate is left out and will be
/// auto generated by the macro.
///
/// Trait support is very basic and experimental, it is unspecified what features (lack of
/// generics and lifetimes) are supported. Try it out, we promise not to break working things
/// (if possible). For trait definitions that are not supported within the unsynn macro it is
/// still possible and advised to define them outside of the macro block.
///
///
/// ## Example
///
/// ```
/// # use unsynn::*;
/// unsynn!{
///     // Marker traits can be defined in a block
///     trait {
///         TestMarker;
///         TestMarker2;
///     }
///
///     // ending with a semicolon, supertraits are supported
///     trait TestMarker3: TestMarker + TestMarker2;
///     // or a brace
///     trait TestMarker4 {}
///
///     // Normal rust syntax for accessor methods
///     trait SimpleAccessor { fn get(&self) -> bool;}
///
///     // simplified trait impl block following the struct definition
///     pub struct SimpleStruct{
///         flag: bool
///     } impl {
///         #[doc = "impl attributes go here"]
///         TestMarker;
///         TestMarker2 {}
///         SimpleAccessor {fn get(&self) -> bool {self.flag}}
///     }
///
///     // normal rust syntax, passthough trait impl
///     impl SimpleAccessor for bool {fn get(&self) -> bool {*self}}
/// }
///
/// // implementing traits outside of the macro works as usual
/// impl TestMarker4 for SimpleStruct {}
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! unsynn {
    (enum $name:ident { $($variant:ident),* }) => {};
    (struct $name:ident { $($member:ident: $parser:ty),* }) => {};
    (struct $name:ident ( $($parser:ty),*);) => {};
    (trait $name:ident;) => {};
    (trait $name:ident{}) => {};
    (impl { $($trait:ident $semicolon_or_block:tt)+ }) => {};
    (impl $trait:ident for $type:ty {$body:tt}) => {};
    (fn $function:ident($params:tt) {$body:tt}) => {};
    (use $($path:path)+ $(as $alias:ident)?) => {};
    (keyword $name:ident = keyword_or_group;) => {};
    (keyword $name:ident != keyword_or_group;) => {};
    (operator $name:ident = "punct";) => {};
}

#[doc(hidden)]
#[cfg(not(doc))]
#[macro_export]
macro_rules! unsynn{
    // enums
    (
        $(#[$attribute:meta])* $pub:vis enum $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {
            $($variants:tt)*
        }
        // PLANNED: impl$(<extragenerics: bounds>)* Trait$(<extra>)
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::unsynn!{
            @enum
            $(#[$attribute])* $pub enum $name
            $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {
                $($variants)*
            }
        }
        $crate::unsynn!{
            @impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
            for $name
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        // next item
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis enum $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {
            $($variants:tt)*
        }
        $($cont:tt)*
    ) => {
        $crate::unsynn!{
            @enum
            $(#[$attribute])* $pub enum $name
            $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {
                $($variants)*
            }
        }
        // next item
        $crate::unsynn!{$($cont)*}
    };
    (
        @enum
        $(#[$attribute:meta])* $pub:vis enum $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {
            $($variants:tt)*
        }
    ) => {
        // The actual enum definition is written as given
        #[derive(Debug)]
        $(#[$attribute])* $pub enum $name
        $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            $($variants)*
        }

        impl$(<$($generic: $crate::Parser $(+ $constraint $(+ $constraints)*)?),*>)? $crate::Parser
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            fn parser(tokens: &mut TokenIter) -> $crate::Result<Self> {
                let mut err = Error::no_error();
                // try to parse each variant
                $crate::unsynn!{@enum_parse_variant(tokens, err) $($variants)*}
                // nothing matched, error out
                Err(err)
            }
        }

        impl$(<$($generic: $crate::ToTokens $(+ $constraint $(+ $constraints)*)?),*>)? $crate::ToTokens
        for $name$(< $($generic),* >)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            fn to_tokens(&self, tokens: &mut $crate::TokenStream) {
                $crate::unsynn!{@enum_to_tokens(self, tokens) {$($variants)*}}
            }
        }
    };

    // normal structs
    (
        $(#[$attribute:meta])* $pub:vis struct $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {
            $($(#[$mattr:meta])* $mpub:vis $member:ident: $parser:ty),* $(,)?
        }
        // PLANNED: impl$(<extragenerics: bounds>)* Trait$(<extra>)
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::unsynn!{
            @struct
            $(#[$attribute])* $pub struct $name
            $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {
                $($(#[$mattr])* $mpub $member: $parser),*
            }
        }
        $crate::unsynn!{
            @impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
            for $name
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        // next item
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis struct $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {
            $($(#[$mattr:meta])* $mpub:vis $member:ident: $parser:ty),* $(,)?
        }
        $($cont:tt)*
    ) => {
        $crate::unsynn!{
            @struct
            $(#[$attribute])* $pub struct $name
            $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {
                $($(#[$mattr])* $mpub $member: $parser),*
            }
        }
        // next item
        $crate::unsynn!{$($cont)*}
    };
    (
        @struct
        $(#[$attribute:meta])* $pub:vis struct $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {
            $($(#[$mattr:meta])* $mpub:vis $member:ident: $parser:ty),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        $(#[$attribute])* $pub struct $name
        $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            $(
                $(#[$mattr])* $mpub $member : $parser
            ),*
        }

        impl$(<$($generic: $crate::Parser $(+ $constraint $(+ $constraints)*)?),*>)? $crate::Parser
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            fn parser(tokens: &mut TokenIter) -> $crate::Result<Self> {
                Ok(Self{$($member: <$parser>::parser(tokens)?),*})
            }
        }

        impl$(<$($generic: $crate::ToTokens $(+ $constraint $(+ $constraints)*)?),*>)? $crate::ToTokens
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            fn to_tokens(&self, tokens: &mut $crate::TokenStream) {
                $(self.$member.to_tokens(tokens);)*
            }
        }
    };

    // tuple structs
    // w/ traits
    (
        $(#[$attribute:meta])* $pub:vis struct $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        (
            $($(#[$mattr:meta])* $mpub:vis $parser:ty),* $(,)?
        )
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?;
        // PLANNED: impl$(<extragenerics: bounds>)* Trait$(<extra>)
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::unsynn!{
            @tuplestruct
            $(#[$attribute])* $pub struct $name
            $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
            (
                $($(#[$mattr])* $mpub $parser),*
            )
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?;
        }
        $crate::unsynn!{
            @impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
            for $name
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    // w/o traits
    (
        $(#[$attribute:meta])* $pub:vis struct $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        (
            $($(#[$mattr:meta])* $mpub:vis $parser:ty),* $(,)?
        )
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?;
        $($cont:tt)*
    ) => {
        $crate::unsynn!{
            @tuplestruct
            $(#[$attribute])* $pub struct $name
            $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
            (
                $($(#[$mattr])* $mpub $parser),*
            )
            $(where
                $($where: $wherebound $(+ $wherebounds)*),*)?;
        }
        $crate::unsynn!{$($cont)*}
    };
    (
        @tuplestruct
        $(#[$attribute:meta])* $pub:vis struct $name:ident
        $(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        (
            $($(#[$mattr:meta])* $mpub:vis $parser:ty),* $(,)?
        )
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?;
    ) => {
        #[derive(Debug)]
        $(#[$attribute])* $pub struct $name
        $(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)?
        ($($(#[$mattr])* $mpub $parser),*)
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?;

        impl$(<$($generic: $crate::Parser $(+ $constraint $(+ $constraints)*)?),*>)? $crate::Parser
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            fn parser(tokens: &mut TokenIter) -> $crate::Result<Self> {
                Ok(Self($(<$parser>::parser(tokens)?),*))
            }
        }

        impl$(<$($generic: $crate::ToTokens $(+ $constraint $(+ $constraints)*)?),*>)? $crate::ToTokens
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {
            fn to_tokens(&self, tokens: &mut $crate::TokenStream) {
                unsynn! {@tuple_for_each item in self : Self($($parser),*) {
                    item.to_tokens(tokens);
                }}
            }
        }
    };

    // type passthough
    (
        $(#[$attribute:meta])* $pub:vis type $name:ident
        $(<$($generic:ident $(: $constraint:ident $(+ $constraints:ident)*)? $(= $default:ty)?),*$(,)?>)?
        = $orig:path;
        $($cont:tt)*
    ) => {
        $(#[$attribute])* $pub type $name$(<$($generic$(: $constraint $(+ $constraints)*)? $(= $default)?),*>)? = $orig;
        // next item
        $crate::unsynn!{$($cont)*}
    };

    // TODO: generics

    // marker trait blocks
    (
        trait {
            $($(#[$attribute:meta])* $pub:vis $name:ident$(: $super:ident $(+ $supers:ident)*)?);+$(;)?
        }
        $($cont:tt)*
    ) => {
        $($(#[$attribute])* $pub trait $name$(: $super $(+ $supers)*)? {})+
        // next item
        $crate::unsynn!{$($cont)*}
    };
    // marker trait definitions with ; at the end
    (
        $(#[$attribute:meta])* $pub:vis trait $name:ident$(: $super:ident $(+ $supers:ident)*)?;
        $($cont:tt)*
    ) => {
        $(#[$attribute])* $pub trait $name$(: $super $(+ $supers)*)? {}
        // next item
        $crate::unsynn!{$($cont)*}
    };
    // trait definition passthrough
    (
        $(#[$attribute:meta])* $pub:vis trait $name:ident$(: $super:ident $(+ $supers:ident)*)?
        {$($body:tt)*}
        $($cont:tt)*
    ) => {
        $(#[$attribute])* $pub trait $name$(: $super $(+ $supers)*)? {$($body)*}
        // next item
        $crate::unsynn!{$($cont)*}
    };

    // impl passthough
    (
        $(#[$attribute:meta])*
        impl$(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)?),*$(,)?>)?
        $trait:ident$(<$($traitgenparam:ident),*$(,)?>)?
        for $type:ty {$($body:tt)*}
        $($cont:tt)*
    ) => {
        $(#[$attribute])*
        impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
        $trait$(<$($traitgenparam),*$(,)?>)?
        for $type {$($body)*}
        // next item
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])*
        impl$(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)?),*$(,)?>)?
        $trait:ident$(<$($traitgenparam:ident),*$(,)?>)?
        for $type:ty;
        $($cont:tt)*
    ) => {
        $(#[$attribute])*
        impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
        $trait$(<$($traitgenparam),*$(,)?>)?
        for $type {}
        // next item
        $crate::unsynn!{$($cont)*}
    };

    // fn passthough
    (
        $(#[$attribute:meta])* $pub:vis fn $name:ident($($param:tt)*) $(-> $ret:ty)? {$($body:tt)*}
        $($cont:tt)*
    ) => {
        $(#[$attribute])* $pub fn $name($($param)?) $(-> $ret)? {$($body)*}
        $crate::unsynn!{$($cont)*}
    };

    // use passthrough
    (use ::$($path:ident)::+$(::{$($inside:tt)+})?; $($cont:tt)*) => {
        use ::$($path)::+$(::{$($inside)+})?;
        $crate::unsynn!{$($cont)*}
    };
    (use $($path:ident)::+$(::{$($inside:tt)+})?; $($cont:tt)*) => {
        use $($path)::+$(::{$($inside)+})?;
        $crate::unsynn!{$($cont)*}
    };
    (use ::$($path:ident)::+ as $alias:ident; $($cont:tt)*) => {
        use $($path)::+ as $alias;
        $crate::unsynn!{$($cont)*}
    };
    (use ::$($path:ident)::+ as $alias:ident; $($cont:tt)*) => {
        use $($path)::+ as $alias;
        $crate::unsynn!{$($cont)*}
    };

    // macro passthrough
    ($macro:ident! {$($args:tt)*} $($cont:tt)*) => {
        $macro! {$($args)*}
        $crate::unsynn!{$($cont)*}
    };
    ($macro:ident! ($($args:tt)*); $($cont:tt)*) => {
        $macro! ($($args)*);
        $crate::unsynn!{$($cont)*}
    };
    ($macro:ident! [$($args:tt)*]; $($cont:tt)*) => {
        $macro! [$($args)*];
        $crate::unsynn!{$($cont)*}
    };

    // keyword delegation
    (
        $(#[$attribute:meta])* $pub:vis keyword $name:ident = $str:literal;
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = $str}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident = $str:literal; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = $str}
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis keyword $name:ident != $str:literal;
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::keyword!{$(#[$attribute])* $pub $name != $str}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident != $str:literal; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name != $str}
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis keyword $name:ident = $group:path;
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = $group}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident = $group:path; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = $group}
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis keyword $name:ident != $group:path;
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::keyword!{$(#[$attribute])* $pub $name != $group}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident != $group:path; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name != $group}
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis keyword $name:ident = [$($keywords:tt),+ $(,)?];
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = [$($keywords),+]}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident = [$($keywords:tt),+ $(,)?]; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name = [$($keywords),+]}
        $crate::unsynn!{$($cont)*}
    };
    (
        $(#[$attribute:meta])* $pub:vis keyword $name:ident != [$($keywords:tt),+ $(,)?];
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::keyword!{$(#[$attribute])* $pub $name != [$($keywords),+]}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis keyword $name:ident != [$($keywords:tt),+ $(,)?]; $($cont:tt)*) => {
        $crate::keyword!{$(#[$attribute])* $pub $name != [$($keywords),+]}
        $crate::unsynn!{$($cont)*}
    };

    // operator delegation
    (
        $(#[$attribute:meta])* $pub:vis operator $name:ident = $str:literal;
        impl {$($(#[$tattr:meta])* $trait:ident $bracesemi:tt)*}
        $($cont:tt)*
    ) => {
        $crate::operator!{$(#[$attribute])* $pub $name = $str}
        $crate::unsynn!{
            @impl
            for $name
            {$({$(#[$tattr])* $trait $bracesemi})*}
        }
        $crate::unsynn!{$($cont)*}
    };
    ($(#[$attribute:meta])* $pub:vis operator $name:ident = $str:literal; $($cont:tt)*) => {
        $crate::operator!{$(#[$attribute])* $pub $name = $str}
        $crate::unsynn!{$($cont)*}
    };

    // terminate recursion
    () => {};

    // @impl muncher
    (
        @impl$(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)?),*>)?
        for $name:ident
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {{$(#[$tattr:meta])* $trait:ident ;} $($cont:tt)*}
    ) => {
        $(#[$tattr])*
        impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)? $trait
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {}
        $crate::unsynn!{
            @impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
            for $name
            {$($cont)*}
        }
    };
    (
        @impl$(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)?),*>)?
        for $name:ident
        $(where
            $($where:ident: $wherebound:ident $(+ $wherebounds:ident)*),*$(,)?)?
        {{$(#[$tattr:meta])* $trait:ident {$($body:tt)*}} $($cont:tt)*}
    ) => {
        $(#[$tattr])*
        impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)? $trait
        for $name$(<$($generic),*>)?
        $(where
            $($where: $wherebound $(+ $wherebounds)*),*)?
        {$($body)*}
        $crate::unsynn!{
            @impl$(<$($generic$(: $constraint $(+ $constraints)*)?),*>)?
            for $name
            {$($cont)*}
        }
    };
    (
        @impl$(<$($generic:ident$(: $constraint:ident $(+ $constraints:ident)*)?),*>)?
        for $name:ident {}
    ) => {};

    // to_tokens for enum tuple variant
    (
        @enum_to_tokens($self:ident, $tokens:ident)
        {
            $(#[$_attrs:meta])* $variant:ident($($tuple:tt)*) $(,$($cont:tt)*)?
        }
    ) => {
        if matches!($self, Self::$variant(..)) {
            unsynn! {@tuple_for_each item in $self : Self::$variant($($tuple)*) {
                item.to_tokens($tokens);
            }}
            return
        }
        $crate::unsynn!{@enum_to_tokens($self, $tokens) {$($($cont)*)?}}
    };

    // to_tokens for enum struct variant
    (
        @enum_to_tokens($self:ident, $tokens:ident)
        {
            $(#[$_attrs:meta])* $variant:ident {
                $($(#[$_mattrs:meta])* $member:ident: $_type:ty),* $(,)?
            }
            $(,$($cont:tt)*)?
        }
    ) => {
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
    (
        @enum_to_tokens($self:ident, $tokens:ident)
        {
            $(#[$_attrs:meta])* $variant:ident
            $(,$($cont:tt)*)?
        }
    ) => {
        if matches!($self, Self::$variant) {
            return
        }
        $crate::unsynn!{@enum_to_tokens($self, $tokens) {$($($cont)*)?}}
    };

    // end recursion
    (@enum_to_tokens($self:ident, $tokens:ident) {}) => {};

    // write for enum tuple variant
    (
        @enum_write($self:ident, $f:ident)
        {
            $(#[$_attrs:meta])* $variant:ident($($tuple:tt)*) $(,$($cont:tt)*)?
        }
    ) => {
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
    (
        @enum_write($self:ident, $f:ident)
        {
            $(#[$_attrs:meta])* $variant:ident $(,$($cont:tt)*)?
        }
    ) => {
        if matches!($self, Self::$variant) {
        }
        $crate::unsynn!{@enum_write($self, $f) {$($($cont)*)?}}
    };

    // end recursion
    (@enum_write($self:ident, $f:ident) {}) => {};

    // Tuple enum variant
    (
        @enum_parse_variant($tokens:ident, $err:ident)
        $(#[$_attrs:meta])* $variant:ident($($tuple:tt)*)
        $(, $($cont:tt)*)?
    ) => {
        if let Ok(parsed) = (|| -> $crate::Result<_> {
            $err.upgrade($crate::unsynn!{@enum_parse_tuple($tokens) $variant($($tuple)*)})
        })() {
            return Ok(parsed);
        }
        $crate::unsynn!{@enum_parse_variant($tokens, $err) $($($cont)*)?}
    };

    // Struct enum variant
    (
        @enum_parse_variant($tokens:ident, $err:ident)
        $(#[$_attrs:meta])*
        $variant:ident{$($members:tt)*}
        $(, $($cont:tt)*)?
    ) => {
        if let Ok(parsed) = (|| -> $crate::Result<_> {
            $err.upgrade($crate::unsynn!{@enum_parse_struct($tokens) $variant{$($members)*}})
        })() {
            return Ok(parsed);
        }
        $crate::unsynn!{@enum_parse_variant($tokens, $err) $($($cont)*)?}
    };

    // Empty enum variant
    (
        @enum_parse_variant($tokens:ident, $err:ident)
        $(#[$_attrs:meta])* $variant:ident
        $(, $($cont:tt)*)?
    ) => {
        /* NOP */
        $crate::unsynn!{@enum_parse_variant($tokens, $err) $($($cont)*)?}
    };

    // end recursion
    (@enum_parse_variant($tokens:ident, $err:ident)) => {};

    // Parse a tuple variant
    (
        @enum_parse_tuple($tokens:ident)
        $variant:ident($($(#[$_attrs:meta])* $parser:ty),* $(,)?)
    ) => {
        $tokens.transaction(
            |mut tokens| Ok(Self::$variant($(<$parser>::parser(&mut tokens)?,)*))
        )
    };

    // Parse a struct variant
    (
        @enum_parse_struct($tokens:ident)
        $variant:ident{$($(#[$_attrs:meta])* $name:ident : $parser:ty),* $(,)?}
    ) => {
        $tokens.transaction(
            |mut tokens| Ok(Self::$variant{$($name : <$parser>::parser(&mut tokens)?,)*})
        )
    };

    // iterate over $variant:($tuple) in $this and apply some $code for each $i
    (
        @tuple_for_each
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

    (
        @tuple_for_each
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
    (@_ $_tt:tt) => {_};
}

/// Define types matching keywords.
///
///  * Any number of attributes (`#[...]`), including documentation comments. Keyword
///    documentation is automatically extended by a small auto generated doc comment listing
///    what a keyword definition will match.
/// * A optional `pub` declaration.
/// * `Name` is the name for the struct to be generated.
/// * `"identifier"` is the case sensitive keyword.
/// * `group` can be a non empty bracketed list of `"identifier"` or any an other keyword
///   definition.
/// * By using `=` the keyword must match the given definition while `!=` negates the output
///   and matches any identifier that is not in the definition.
///
/// `Name::parse()` will then only match the defined identifier.  It will implement `Debug`
/// and `Clone` for keywords. Additionally `AsRef<str>` is implemented for each Keyword
/// to access the identifier string from rust code.
///
/// The `unsynn!` macro supports defining keywords by using `keyword Name = "ident";`, the
/// `pub` specification has to come before `keyword` then.
///
/// In case a invalid keyword is defined (not an identifier) the compilation will panic. But
/// because the actual matching function is optimized and lazy evaluated this will only happen
/// on the first use of the invalid keyword definition.
///
/// Keywords implement `AsRef<str>`, `AsRef<Ident>` and `Keyword::as_str(&self) -> &str`. For
/// Keywords that are defined with a single literal string (`keyword!{ Foo = "foo"}`) the
/// `Default` trait is implemented. Thus they can be created and inserted statically.
///
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// keyword!{
///     /// Optional documentation for `If`
///     pub If = "if";
///     pub Else = "else";
///     // keywords can be grouped from existing keywords
///     IfElse = [If, Else,];
///     // or contain identifiers in double quotes
///     IfElseThen = [IfElse, "then"];
///     // matching can be negated with `!=`
///     NotIfElseThen != [IfElse, "then"];
/// }
///
/// let mut tokens = "if".to_token_iter();
/// let if_kw = If::parse(&mut tokens).unwrap();
/// assert_eq!(if_kw.as_str(), "if");
/// # let mut tokens = "else if then something".to_token_iter();
/// # let else_kw = Else::parse(&mut tokens).unwrap();
/// # assert_eq!(else_kw.as_str(), "else");
/// # let ifelse_kw = IfElse::parse(&mut tokens).unwrap();
/// # assert_eq!(ifelse_kw.as_str(), "if");
/// # let ifelsethen_kw = IfElseThen::parse(&mut tokens).unwrap();
/// # assert_eq!(ifelsethen_kw.as_str(), "then");
/// # let notifelsethen_kw = NotIfElseThen::parse(&mut tokens).unwrap();
/// # assert_eq!(notifelsethen_kw.as_str(), "something");
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! keyword {
    ($name:ident = $str:literal; ...) => {};
    ($name:ident = $group:path; ...) => {};
    ($name:ident = [$($keywords:tt),+]; ...) => {};
    ($name:ident != $str:literal; ...) => {};
    ($name:ident != $group:path; ...) => {};
    ($name:ident != [$($keywords:tt),+]; ...) => {};
}

#[doc(hidden)]
#[cfg(not(doc))]
#[macro_export]
macro_rules! keyword{
    ($(#[$attribute:meta])* $pub:vis $name:ident = $str:literal $(;$($cont:tt)*)?) => {
        $crate::keyword!{
            @{} $(#[$attribute])* $pub $name [$str]
        }
        // implement `Default` for single token keywords
        $crate::keyword!{
            @default $name $str
        }
        $crate::keyword!{$($($cont)*)?}
    };
    ($(#[$attribute:meta])* $pub:vis $name:ident != $str:literal $(;$($cont:tt)*)?) => {
        $crate::keyword!{
            @{!} $(#[$attribute])* $pub $name [$str]
        }
        $crate::keyword!{$($($cont)*)?}
    };
    ($(#[$attribute:meta])* $pub:vis $name:ident = $group:path $(;$($cont:tt)*)?) => {
        $crate::keyword!{
            @{} $(#[$attribute])* $pub $name [$group]
        }
        $crate::keyword!{$($($cont)*)?}
    };
    ($(#[$attribute:meta])* $pub:vis $name:ident != $group:path $(;$($cont:tt)*)?) => {
        $crate::keyword!{
            @{!} $(#[$attribute])* $pub $name [$group]
        }
        $crate::keyword!{$($($cont)*)?}
    };
    (
        $(#[$attribute:meta])*
        $pub:vis $name:ident = [$($keywords:tt),+ $(,)?]
        $(;$($cont:tt)*)?
    ) => {
        $crate::keyword!{
            @{} $(#[$attribute])* $pub $name [$($keywords),+]
        }
        $crate::keyword!{$($($cont)*)?}
    };
    (
        $(#[$attribute:meta])*
        $pub:vis $name:ident != [$($keywords:tt),+ $(,)?]
        $(;$($cont:tt)*)?
    ) => {
        $crate::keyword!{
            @{!} $(#[$attribute])* $pub $name [$($keywords),+]
        }
        $crate::keyword!{$($($cont)*)?}
    };
    (
        @{$($not:tt)?} $(#[$attribute:meta])*
        $pub:vis $name:ident [$($keywords:tt),+]
        $(;$($cont:tt)*)?
    ) => {
        $(#[$attribute])*
        // TODO $crate::docgen!{keyword {$($not)?} {$keywords}},
        #[doc = concat!(
             $crate::docgen!{@keyword_header $($not)?},
             $($crate::docgen!{@keyword_doc $keywords}),+
        )]
        #[derive(Debug, Clone, PartialEq, Eq)]
        $pub struct $name($crate::CachedIdent);

        impl $crate::Parser for $name {
            fn parser(tokens: &mut $crate::TokenIter) -> $crate::Result<Self> {
                use $crate::Parse;
                let at = tokens.clone().next();
                $crate::CachedIdent::parse_with(tokens, |ident, tokens| {
                    if $($not)? Self::matches(ident.as_str()) {
                        Ok($name(ident))
                    } else {
                        $crate::Error::other::<$name>(
                            at,
                            tokens,
                            format!(
                                "keyword for {:?} expected, got {:?} at {:?}",
                                stringify!($name),
                                ident.as_str(),
                                ident.span().start()
                            ),
                        )
                    }
                }).refine_err::<Self>()
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut $crate::TokenStream) {
                self.0.to_tokens(tokens);
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }

        impl AsRef<$crate::Ident> for $name {
            fn as_ref(&self) -> &$crate::Ident {
                &*self.0
            }
        }

        impl $name {
            /// get the underlying `&str` from a keyword
            #[allow(dead_code)]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[doc(hidden)]
            #[inline]
            pub const fn keywords() -> &'static $crate::KeywordGroup {
                static KEYWORDS: $crate::KeywordGroup = $crate::keyword! {@group $($keywords),+};
                &KEYWORDS
            }

            fn matches(this: &str) -> bool {
                static MATCHFN: std::sync::LazyLock<Box<dyn Fn(&str) -> bool + Send + Sync>> =
                    std::sync::LazyLock::new(|| $crate::create_matchfn($name::keywords()));
                MATCHFN(this)
            }
        }

        $crate::keyword!{$($($cont)*)?}
    };
    () => {};

    (@default $name:ident $str:literal) => {
        impl Default for $name {
            fn default() -> Self {
                Self(CachedIdent::parse(&mut $str.to_token_iter()).unwrap())
            }
        }
    };

    // keyword group creation
    (@group $($entry:tt),+) => {
        $crate::KeywordGroup::List(
            &[$(&$crate::keyword!{@entry $entry}),+]
        )
    };
    (@entry $kw:literal) => {
        $crate::KeywordGroup::Keyword($kw)
    };
    (@entry $sub:path) => {
        *<$sub>::keywords()
    };
}

/// Define types matching operators (punctuation sequences).
///
/// `operator!{ pub Op = "punct"; ...}`
///
/// * A optional `pub` defines the operators public, default is private
/// * `Op` is the name for the struct to be generated
/// * `"punct"` is up to 4 ASCII punctuation characters
///
/// `Op::parse()` will match the defined operator. It will implement `Debug` and `Clone`
/// for operators.
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
/// assert_tokens_eq!(wl, str "<~~");
/// let wr = WRArrow::parse(&mut tokens).unwrap();
/// assert_tokens_eq!(wr, str "~~>");
/// # let wr = WRArrow::parse(&mut tokens).unwrap();
/// # assert_tokens_eq!(wr, "~~>");
/// # let wl = WLArrow::parse(&mut tokens).unwrap();
/// # assert_tokens_eq!(wl, "<~~");
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
        // TODO $crate::docgen!{operator $op},
        #[doc = $crate::docgen!{@operator_doc $op}]
        $pub type $name = $crate::Operator<
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

// The documentation generator
#[cfg(not(feature = "docgen"))]
#[doc(hidden)]
#[macro_export]
macro_rules! docgen {
    // just dispose anything
    ($($_:tt)*) => {
        ""
    };
}

#[cfg(feature = "docgen")]
#[doc(hidden)]
#[macro_export]
macro_rules! docgen {
    (@keyword_header) => {
        "Matches: "
    };
    (@keyword_header !) => {
        "Matches any `Ident` but: "
    };
    (@keyword_doc $kw:literal) => {
        concat!("`", $kw, "`, ")
    };
    (@keyword_doc $sub:path) => {
        concat!("[`", stringify!($sub), "`], ")
    };
    (@operator_doc $op:literal) => {
        concat!("`", $op, "`")
    };
}

/// unsynn provides its own `quote!{}` macro that translates tokens into a `TokenStream` while
/// interpolating variables prefixed with a `Pound` sign (#). This is similar to what the quote macro from
/// the quote crate does but not as powerful. There is no `#(...)` repetition (yet).
///
/// Instead we provide `#{...}` blocks which must return a `IntoIterator` whose items
/// implement `ToTokens`. When blocks returns a single value just wrap this in `Some()`
/// because `Option` implements the necessary `IntoIterator`.
///
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// let ast = <Cons<ConstInteger<1>, Plus, ConstInteger<2>>>::default();
/// let quoted = quote! { let a = #ast;};
/// assert_tokens_eq!(quoted, "let a = 1+2;");
///
/// // or using #{...} blocks
/// let quoted = quote! {
///     let a = #{Some(<Cons<ConstInteger<1>, Plus, ConstInteger<2>>>::default())};
/// };
/// assert_tokens_eq!(quoted, "let a = 1+2;");
/// ```
#[macro_export]
macro_rules! quote {
    ($($tokens:tt)*) => {
        {
            let mut tokenstream = $crate::TokenStream::new();
            $crate::quote_intern!{tokenstream $($tokens)*};
            tokenstream
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! quote_intern {
    ($tokenstream:ident #$var:ident $($rest:tt)*) => {
        $var.to_tokens(&mut $tokenstream);
        $crate::quote_intern!{$tokenstream $($rest)*}
    };
    ($tokenstream:ident #{$($code:tt)*} $($rest:tt)*) => {
        {$($code)*}.into_iter().for_each(|i| i.to_tokens(&mut $tokenstream));
        $crate::quote_intern!{$tokenstream $($rest)*}
    };
    // hash followed by parenthesis is reserved
    ($tokenstream:ident #($($reserved:tt)*) $($rest:tt)*) => {
        compile_error!("#(...) reserved for future")
    };
    ($tokenstream:ident ( $($nested:tt)* ) $($rest:tt)*) => {
        $tokenstream.extend(
            Some(
                $crate::TokenTree::Group(
                    $crate::Group::new($crate::Delimiter::Parenthesis, $crate::quote!{$($nested)*})
                )
            ).into_iter()
        );
        $crate::quote_intern!{$tokenstream $($rest)*}
    };
    ($tokenstream:ident { $($nested:tt)* } $($rest:tt)*) => {
        $tokenstream.extend(
            Some(
                $crate::TokenTree::Group(
                    $crate::Group::new($crate::Delimiter::Brace, $crate::quote!{$($nested)*})
                )
            ).into_iter()
        );
        $crate::quote_intern!{$tokenstream $($rest)*}
    };
    ($tokenstream:ident [ $($nested:tt)* ] $($rest:tt)*) => {
        $tokenstream.extend(
            Some(
                $crate::TokenTree::Group(
                    $crate::Group::new($crate::Delimiter::Bracket, $crate::quote!{$($nested)*})
                )
            ).into_iter()
        );
        $crate::quote_intern!{$tokenstream $($rest)*}
    };

    ($tokenstream:ident $token:tt $($rest:tt)*) => {
        let t: $crate::TokenStream = std::str::FromStr::from_str(stringify!($token)).unwrap();
        $tokenstream.extend(Some(t).into_iter());
        $crate::quote_intern!{$tokenstream $($rest)*}
    };

    ($tokenstream:ident) => {};
}
