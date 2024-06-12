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


