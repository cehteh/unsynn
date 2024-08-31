unsynn (from german 'unsinn' for nonsense) is a minimalist rust parser library. It achieves
this by leaving out the actual grammar implementations and compromise on simpler error
reporting. In exchange it offers simple composeable Parsers and ergonomic Parser
construction. Grammars will be implemented in their own crates (see unsynn-rust).

It is primarily intended use is when one wants to create proc macros for rust that define their
own grammar or need only sparse rust parsers.


# Example

```rust
# use unsynn::*;
let mut token_iter = quote::quote!{ foo ( bar, baz, barf ) }.into_iter();

// Composition
let ast =
    Cons::<Ident, ParenthesisGroupContaining::<CommaDelimitedVec<Ident>>>
        ::parse(&mut token_iter).unwrap();

// The same defining a custom type, the macro will generate the `Parser` and `ToToken` impls.
unsynn!{
    struct IdentThenParenthesisedIdents {
        ident: Ident,
        pidents: ParenthesisGroupContaining::<CommaDelimitedVec<Ident>>,
    }
}

let mut token_iter = quote::quote!{ foo ( bar, baz, barf ) }.into_iter();

let ast = IdentThenParenthesisedIdents::parse(&mut token_iter).unwrap();
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
