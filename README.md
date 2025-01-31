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

Notice that unsynn implements `Parser` and `ToTokens` for many standard rust types. Like
we use `u32` in this example.

```rust
# use unsynn::*;
let mut token_iter = "foo ( 1, 2, 3 )".to_token_iter();

unsynn!{
    struct IdentThenParenthesisedNumbers {
        ident: Ident,
        numbers: ParenthesisGroupContaining::<CommaDelimitedVec<u32>>,
    }
}

// iter.parse() is from the IParse trait
let ast: IdentThenParenthesisedNumbers = token_iter.parse().unwrap();

assert_eq!(
    ast.tokens_to_string(),
    "foo(1,2,3)".tokens_to_string()
)
```

## Using Composition

Composition can be used without defining new datatypes. This is useful for simple parsers or
when one wants to parse things on the fly which are desconstructed immediately.

```rust
# use unsynn::*;
// We parse this below
let mut token_iter = "foo ( 1, 2, 3 )".to_token_iter();

// Type::parse() is from the Parse trait
let ast =
    Cons::<Ident, ParenthesisGroupContaining::<CommaDelimitedVec<u32>>>
        ::parse(&mut token_iter).unwrap();

assert_eq!(
    ast.tokens_to_string(),
    "foo ( 1, 2, 3 )".tokens_to_string()
)
```

## Custom Operators and Keywords

To define keywords and operators we provide the `keyword!` and `operator!` macros:

```rust
# use unsynn::*;
keyword! {
    pub Calc = "CALC";
}

operator! {
    pub Add = "+";
    pub Substract = "-";
    pub Multiply = "*";
    pub Divide = "/";
}

// The above can be written within a unsynn! macro as:
// unsynn! {
//     pub keyword Calc = "CALC";
//     pub operator Add = "+";
//     pub operator Substract = "-";
//     pub operator Multiply = "*";
//     pub operator Divide = "/";
// }

// looks like BNF, but can't do recursive types
type Expression = Cons<Calc, AdditiveExpr, Semicolon>;
type AdditiveOp = Either<Add, Substract>;
type AdditiveExpr = Either<Cons<MultiplicativeExpr, AdditiveOp, MultiplicativeExpr>, MultiplicativeExpr>;
type MultiplicativeOp = Either<Multiply, Divide>;
type MultiplicativeExpr = Either<Cons<LiteralInteger, MultiplicativeOp, LiteralInteger>, LiteralInteger>;

let ast = "CALC 2*3+4/5 ;".to_token_iter()
    .parse::<Expression>().expect("syntax error");
```


# Feature Flags

By default unsynn is very lean and does not include extra features. The only thing that are
always present are the [`Parser`], [`Parse`] and [`ToTokens`] traits.  The following features
enable extra traits:

- **impl_debug**  
  Unsynn generates Debug impls only in debug builds (`debug_assertions` is set). This flag
  adds [`Debug`](std::fmt::Debug) implementations to generic unsynn types in release builds as
  well.

- **impl_display**  
  Adds [`Display`](std::fmt::Display) implementations to generic unsynn types.
  `ToTokens::tokens_to_string() -> String` may be preferable in many cases.

Note that `Display` can't be implemented for all types (eg. [`Option`]). Further `Display` may
sometimes be surprising since we do not have good rules how to pretty-print tokens (eg. spaces
around Delimiters). Display then often inserts surplus spaces to ensure that tokens are
properly delimited.
