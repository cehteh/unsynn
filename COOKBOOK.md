# Cookbook

## Parsing

Parsing is done over a `<TokenStream as IntoIterator>::IntoIter` which is shortened as
[`TokenIter`].

The main trait for parsing a [`TokenIter`] is the [`Parse`] trait. This traits methods are all
default implemented and can be used as is. [`Parse`] is implemented for all types that
implement [`Parser`] and [`ToTokens`]. [`Parser`] is the trait that has to be implemented for
each type that should be parsed.

The [`IParse`] trait is implemented for [`TokenIter`], this calls `Parse::parse()` in a
convenient way when types can be inferred.

[`ToTokens`] complements [`Parser`] as it will turn any parseable entity back into a
`TokenStream`.


### [`Parse::parse_with()`] transformations

The `Parse::parse_with()` method is used for parsing in more complex situations. In the
simplest case it can be used to validate the values of a parsed type. More complex usage will
fill in `HiddenState` and other not parsed members or construct completely new types from
parsed entities.


## Composition and Type Aliases

For moderately complex types it is possible to use composition with [`Cons`], [`Either`] and
other container types instead defining new enums or structures.

It is recommended to alias such composed types to give them useful names. This can be used for
creating grammars on the fly without any boilerplate code.


## The [`unsynn!{}`] Macro

The easiest way to describe grammars is to use the [`unsynn!{}`] macro. This allows to define
grammars just by defining enums and structures. The macro will generate the necessary
implementations for [`Parser`] and [`ToTokens`] but offers no flexibility in what is
generated. It is possible to add [`HiddenState<T>`] members to add non syntactic entries to
there custom structs.


## Implementing Parsers

### Transactions

The [`Parse`] Trait parses items within a transaction. This is with the
[`Transaction::transaction()`] method. Internally this clones the iterator, calls the
[`Parser::parser()`] method and copies the cloned iterator back to the original on success.
This means that if a parser fails, the input is reset to the state before the parser was
called. For efficiency reasons the [`Parser::parser()`] methods are themself not
transactional, when they fail they leave the input in a consumed state.

When one wants to manually parse *alternatives* within a `Parser` (like in a enum) it must be
manually called within a transaction. This is only necessary when the parsed entity is
compound and not the final alternative. For simple parsable entities one can just call the
`parse()` method which already provides the transaction.

```rust
# use unsynn::*;
enum MyEnum {
    // Complex variant
    Tuple(i32, i32),
    // Simple variant
    Simple(i32),
    // Another simple variant
    Another(Ident),
}

impl Parser for MyEnum {
    fn parser(input: &mut TokenIter) -> Result<Self> {
        // Use [`Transaction::transaction()`] to parse the tuple variant
        if let Ok(tuple) = input.transaction(
            |mut trans_input|
            Ok(MyEnum::Tuple(
                i32::parser(&mut trans_input)?,
                i32::parser(&mut trans_input)?,
            ))
        ) {
            Ok(tuple)
        } else
        // Try to parse the simple variant
        // can use the `Parse::parse()` or `IParse::parse()` method directly since a
        // single entity will be put in a transaction by those.
        if let Ok(i) = input.parse() {
            Ok(MyEnum::Simple(i))
        } else {
            // Try to parse the last variant
            // this can use the `Parser::parser()` method since this is the final alternative
            Ok(MyEnum::Another(Ident::parser(input)?))
        }
    }
}
```


### Different ways to implement Parsers

There are different approaches how one can implement parsers. Each has its own advantages and
disadvantages. unsynn supports both, so one can freely mix whatever makes most sense in a
particular case.

unsynn uses a rather simple, first come - first served approach when parsing. Parsers may be a
subset or share a common prefix with other parsers. This needs some attention.

In the case where parsers are subsets to other parsers and one puts them into a disjunction in
a `unsynn! { enum ..}` or in a `Either` combinator the more specific case must come first,
otherwise it will never match.

```rust
# use unsynn::*;
// Ident must come first since TokenTree matches any token.
type Example = Either<Ident, TokenTree>;
```

For the other case where parsers sharing longer prefixes (this should rarely happen in
practice) it may benefit performance to break these into a type with the shared prefix that
dispatches on the distinct parts.


#### Exact AST Representation

One approach is to define a structures that reflects the AST of the grammar exactly.  This is
what the [`unsynn!{}`] macro and composition does. The program later works with the parsed
structure directly. The advantage is that [`Parser`] and [`ToTokens`] are simple and come for
free and that the source structure of the AST stays available.

```rust
# use unsynn::*;

unsynn!{
    // define a list of Ident = "LiteralString",.. assignments
    struct Assignment {
        id: Ident,
        _equal: Assign,
        value: LiteralString,
    }

    struct AssignmentList {
        list: DelimitedVec<Assignment, Comma>
    }
}
```

When the implementation generated by the [`unsynn!{}`] macro is not sufficient one can
implement [`Parser`] and [`ToTokens`] for custom structs and enums  manually.


#### High level representation

Another approach is to represent the data more in the way further processing requires. This
simplifies working with the data but one **has** to implement the [`Parser`] and [`ToTokens`]
traits manually. Sometimes the [`Parse::parse_with()`] method will become useful in such
cases.

```rust
# use unsynn::*;
# use std::collections::HashMap;
// We could go with `unsynn!{struct Assignment{...}}` as above here. But lets use composition
// as example here. This stays internal so its complexity isnt exposed.
type Assignment = Cons<Ident, Assign, LiteralString>;

// Here we'll parse the list of assignments into a structure that represents the
// data in a way thats easier to use from a rust program
#[derive(Default)]
struct AssignmentList {
    // each 'Ident = LiteralString'
    list: Vec<(Ident, String)>,
    // We want to have a fast lookup to the entries
    lookup: HashMap<Ident, usize>,
}

impl Parser for AssignmentList {
    fn parser(input: &mut TokenIter) -> Result<Self> {
        let mut assignment_list = AssignmentList::default();

        // We construct the `AssignmentList` by parsing the content, appending and processing it.
        while let Ok(assignment) = Delimited::<Assignment, Comma>::parse(input) {
            assignment_list.list.push((
                assignment.value.first.clone(),
                // Create a String without the enclosing double quotes
                assignment.value.third.as_str().to_string()
            ));
            // add it to the lookup
            assignment_list.lookup.insert(
                assignment.value.first.clone(),
                assignment_list.list.len()-1
            );
            // No Comma, no more assignments
            if assignment.delimiter.is_none() {
                break;
            }
        }
        Ok(assignment_list)
    }
}

impl ToTokens for AssignmentList {
    fn to_tokens(&self, output: &mut TokenStream) {
        for a in &self.list {
            a.0.to_tokens(output);
            Assign::new().to_tokens(output);
            LiteralString::from_str(&a.1).to_tokens(output);
            Comma::new().to_tokens(output);
        }
    }
}
# fn test_assignment_list() {
#     let mut input = r#"a = "b", c = "d""#.to_token_iter();
#     let parsed = AssignmentList::parse(&mut input).unwrap();
#     assert_eq!(parsed.list.len(), 2);
#     assert_eq!(parsed.list[1].1, "d");
# }
```
