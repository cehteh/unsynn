unsynn (from german 'unsinn' for nonsense) is a minimalist rust parser library. It achives
this by leaving out the parsers for the rust grammar and compromise on error reporting. In
exchange it offers simple composeable Parsers.

Its intended use is when one wants to create proc macros for rust that define their own
grammar.

# Example

```rust
# use unsynn::*;
let mut token_iter = quote::quote!{ foo ( bar, baz, barf ) }.into_iter();

let ast =
    Cons::<Ident, ParenthesisGroupContaining::<CommaDelimitedVec<Ident>>>
        ::parse(&mut token_iter).unwrap();
```
