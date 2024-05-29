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
                    $(
                        if let Ok(parsed) = <$parse>::parse(tokens) {
                            return Ok($name::$variant(parsed));
                        }
                    )*
                        match tokens.next() {
                            Some(token) => $crate::Error::unexpected_token(token),
                            None => $crate::Error::unexpected_end()
                        }
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

/// Define types matching keywords.
///
/// `keyword!{ Name = "identifier", ...}`
///
/// * `Name` is the name for the (`struct Name(Cached<Ident>)`) to be generated
/// * `"identifier"` is the case sensitive keyword
///
/// `Name::parse()` will then only match the defined identifier.  Additionally `AsRef<str>` is
/// implemented for each Keyword to access the identifier string from rust code.
///
/// # Example
///
/// ```
/// # use unsynn::*;
/// keyword!{
///     If = "if",
///     Else = "else",
/// }
///
/// let tokens = quote::quote!{ if else };
/// let mut iter = tokens.into_iter();
/// let if_kw = If::parse(&mut iter).unwrap();
/// assert_eq!(if_kw.as_ref(), "if");
/// let else_kw = Else::parse(&mut iter).unwrap();
/// assert_eq!(else_kw.as_ref(), "else");
/// ```
#[macro_export]
macro_rules! keyword{
    ($($name:ident = $str:literal),*$(,)?) => {
        $(
            pub struct $name($crate::CachedIdent);

            impl Parser for $name {
                fn parser(tokens: &mut TokenIter) -> Result<Self> {
                    Ok(Self($crate::CachedIdent::parse_with(tokens, |keyword| {
                        if keyword == $str {
                            Ok(keyword)
                        } else {
                            $crate::Error::other::<$crate::CachedIdent>(
                                format!(
                                    "keyword {:?} expected, got {:?} at {:?}",
                                    $str,
                                    keyword.string(),
                                    keyword.span().start()
                                )
                            )
                        }
                    })?))
                }
            }

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    self.0.as_ref()
                }
            }
        )*
    }
}
