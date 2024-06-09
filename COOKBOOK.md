# Cookbook

## Composition and Type Aliases

For moderately complex types we use composition with [`Cons`], [`Either`] and other container
types instead defining enums or structures in detail.

Such composed types are frequently aliased to give them handy names.

This can be employed in user code as well to crate grammars on the fly without any boilerplate code.

## The [`unsynn!{}`] Macro

The [`unsynn!{}`] macro is for parsing grammar entities that can be straightforward written as
tuple, structure or enum. It does all the necessary implementaitions for the user given entity
but offers no flexibility in what is generated. It is possible to add [`HiddenState<T>`]
members to add non syntactic entries to there custom structs.

