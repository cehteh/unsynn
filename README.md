unsynn (from german 'unsinn' for nonsense) is a minimalist rust parser library. It achieves
this by leaving out the actual grammar implementations and compromise on simpler error
reporting. In exchange it offers simple composeable Parsers and ergonomic Parser
construction. Grammars will be implemented in their own crates (see unsynn-rust).

It is primarily intended use is when one wants to create proc macros for rust that define their
own grammar or need only sparse rust parsers.


# Examples

## Custom Types

The `unsynn!{}` macro will generate the `Parser` and `ToToken` impls (and more).  This is
optional, the impls could be written by hand when necessary.

Notice that unsynn can implements `Parser` and `ToTokens` for many standard rust types. Like
we `u32` in this example.

```rust
# use unsynn::*;
let mut token_iter = quote::quote!{
    foo ( 1, 2, 3 )
}.into_iter();

unsynn!{
    struct IdentThenParenthesisedIdents {
        ident: Ident,
        pidents: ParenthesisGroupContaining::<CommaDelimitedVec<u32>>,
    }
}

let ast = IdentThenParenthesisedIdents::parse(&mut token_iter).unwrap();

assert_eq!(
    ast.to_token_stream().to_string(),
    quote::quote!{foo ( 1, 2, 3 )}.to_string()
)
```

## Using Composition

Composition can be used without defining new datatypes. This is useful for simple parsers or
when one wants to parse things on the fly which are desconstructed immediately.

```rust
# use unsynn::*;
let mut token_iter = quote::quote!{
    // We parse this below
    foo ( 1, 2, 3 )
}.into_iter();

let ast =
    Cons::<Ident, ParenthesisGroupContaining::<CommaDelimitedVec<u32>>>
        ::parse(&mut token_iter).unwrap();

assert_eq!(
    ast.to_token_stream().to_string(),
    quote::quote!{foo ( 1, 2, 3 )}.to_string()
)
```


# Feature Flags

By defaut unsynn is very lean and does not include extra features. The only thing that are
always present are the [`Parser`], [`Parse`] and [`ToTokens`] traits.  The following features
enable extra traits:

- **impl_debug**  
  Adds [`Debug`](std::fmt::Debug) implementations to generic unsynn types.

- **impl_display**  
  Adds [`Display`](std::fmt::Display) implementations to generic unsynn types.

Note that `Display` can't be implemented for all types (eg. [`Option`]). Further `Display` may
sometimes be surprising since we do not have good rules how to pretty-print tokens (eg. spaces
around Delimiters). Display then often inserts surplus spaces to ensure that tokens are
properly delimited.
