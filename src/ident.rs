use crate::*;

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
                            Some(keyword)
                        } else {
                            None
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
