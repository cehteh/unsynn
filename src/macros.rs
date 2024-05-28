/// Construct types with a `Parser` implementation that will try to parse each variant in
/// order. This macro supports enums, tuple structs and normal structs. Generics are not
/// supported and all entities in a single `unsynn!` invocation have to be of the same kind.
///
/// # Examples
///
/// * Enum
/// ```
/// # use unsynn::*;
/// unsynn!{
///     enum MyEnum {
///         Ident(Ident),
///         Braced(BraceGroup),
///         Text(LiteralString),
///         Number(LiteralInteger),
///     }
/// }
///
/// let mut token_iter = quote::quote! { ident { within brace } "literal string" 1234}.into_iter();
///
/// let MyEnum::Ident(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// let MyEnum::Braced(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// let MyEnum::Text(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// let MyEnum::Number(_) = MyEnum::parse(&mut token_iter).unwrap() else { panic!()};
/// ```
///
/// * Struct
/// ```
/// # use unsynn::*;
/// unsynn!{
///     struct MyStruct {
///         text: LiteralString,
///         number: LiteralInteger,
///     }
/// }
///
/// let mut token_iter = quote::quote! { "literal string" 1234 }.into_iter();
///
/// let my_struct =  MyStruct::parser(&mut token_iter).unwrap();
/// ```
///
/// * Tuple Struct
/// ```
/// # use unsynn::*;
/// unsynn!{
///     struct MyTupleStruct(Ident, LiteralString);
/// }
///
/// let mut token_iter = quote::quote! { ident "literal string"}.into_iter();
///
/// let my_tuple_struct =  MyTupleStruct::parser(&mut token_iter).unwrap();
/// ```
#[macro_export]
macro_rules! unsynn{
    ($($(#[$attribute:meta])* $pub:vis enum $name:ident { $($variant:ident($parse:ty)),* $(,)? })*) => {
        $(
            $(#[$attribute])* $pub enum $name {
                $($variant($parse)),*
            }

            impl Parser for $name {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    $(if let Ok(parsed) = <$parse>::parse(tokens) {
                        return Ok($name::$variant(parsed));
                    })*
                        Err(format!("neither of {} matched", stringify!($name)).into())
                }
            }
        )*
    };
    ($($(#[$attribute:meta])* $pub:vis struct $name:ident { $($member:ident: $parse:ty),* $(,)? })*) => {
        $(
            $(#[$attribute])* $pub struct $name {
                $($member : $parse),*
            }

            impl Parser for $name {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    Ok(Self{$($member: <$parse>::parser(tokens)?),*})
                }
            }
        )*
    };
    ($($(#[$attribute:meta])* $pub:vis struct $name:ident ($($parse:ty),* $(,)?);)*) => {
        $(
            $(#[$attribute])* $pub struct $name (
                $($parse),*
            );

            impl Parser for $name {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    Ok(Self($(<$parse>::parser(tokens)?),*))
                }
            }
        )*
    };
}
