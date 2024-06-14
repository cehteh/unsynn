# Cookbook

## Composition and Type Aliases

For moderately complex types we use composition with [`Cons`], [`Either`] and other container
types instead defining enums or structures in detail.

Such composed types are frequently aliased to give them handy names. This can be used in user
code as well creating grammars on the fly without any boilerplate code.

## The [`unsynn!{}`] Macro

The [`unsynn!{}`] macro is for parsing grammar entities that can be straightforward written as
tuple, structure or enum. It does all the necessary implementations for the user given entity
but offers no flexibility in what is generated. It is possible to add [`HiddenState<T>`]
members to add non syntactic entries to there custom structs.

## [`Parse::parse_with()`] transformations

The `Parse::parse_with()` method is used for parsing in more complex situations. In the
simplest case it can be used to validate the values of a parsed type. More complex usage will
fill in `HiddenState` and other not parsed members or construct completely new types from
parsed entities.


## How to Parse

There are generally two approaches how one can implement parsers. Each has its own advantages
and disadvantages. unsynn supports both, so one can freely mix whatever makes most sense in a
particular case.

Moreover there are two API's how `parse()` can be called. At the basic level every type that
implements `Parser` gets `Parse` implemented as well thus making `T::parse(&mut TokenIter)`
available. For convenience we also have a `IParse` trait implemented for `TokenIter` thus one
can call `let parsed = some_tokeniter.parse::<T>()?;` this is especially useful when the type
can be inferred as it won't need the turbofish notation. 


### Exact AST Representation

One approach is to define a structures that reflects the AST of the grammar exactly.  This is
what the [`unsynn!{}`] macro and composition does. The program later works with the parsed
structure directly. The advantage is that `Parser` and `ToToken` are simple and come for free
and that the source structure of the AST stays available.

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

### High level representation

Another approach is to represent the data more in the way further processing requires. This
simplifies working with the data but one has to implement the `Parser` and `ToToken` traits
manually. Sometimes the `Parse::parse_with()` method will become useful in such cases.

```rust
# use unsynn::*;
# use std::collections::HashMap;

// We could go with `unsynn!{struct Assignment{...}}` as above here. But lets use composition
// as example here. This stays internal so its complexity isnt exposed.
type Assignment = Cons<Ident, Cons<Assign, LiteralString>>;

// Here we'll parse the list of assignments into a structure that represents the
// data in a way thats easier to use from a rust program
#[derive(Default)]
struct AssignmentList {
    // each 'Ident = LiteralString'
    list: Vec<(Ident, String)>,
    // We want to be able to have a fast lookup to the entries
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
                assignment.value.second.second.as_str().to_string()
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
        }
    }
}
```
