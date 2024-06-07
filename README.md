unsynn (from german 'unsinn' for nonsense) is a minimalist rust parser library. It achives
this by leaving out the actual grammar implementations and compromise on simpler error
reporting. In exchange it offers simple composeable Parsers and ergonomic Parser
construction. Grammars will be implemented in their own crates (see unsynn-rust).

It is primarly intended use is when one wants to create proc macros for rust that define their
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


# Features

By defaut unsynn is very lean and does not include extra features. The only thing that are
always present are the `Parser`, `Parse` and `ToToken` traits.  The following features 
enable extra traits:

- **impl_debug**  
  Adds `Debug` implementations to generic unsynn types.

- **impl_display**  
  Adds `Display` implementations to generic unsynn types.

Note that `Display` can't be implemented for some std types (eg. `Option`). Further `Display`
may sometimes be surprising since we do not have rules how to pretty-print tokens (eg. spaces
around Delimiters)
