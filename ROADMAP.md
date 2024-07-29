The 0.0.x series are *very* unstable and may break the API with every commit, there will be
frequent releases to get feedback. Since 0.0.6 it goes into some battle testing for writing a
proc macro crate that does not need to parse rust syntax. Once that is settled a initial 0.1.0
release will be made with a more stable API. From there on the planned 'unsynn-rust' and along
that a 'unsynn-derive' will be implemented. When the later two are working and no major
deficiencies in 'unsynn' are found then it is time for a 1.0.0 release.


## Planned/Ideas

* Some types are currently implemented as ZST's. Drawback is that this looses the relation to
  the source code they come from. Eventually it should be reconsidered and maybe add a 'spans'
  feature that turns ZST's to contain a `Span`.
* Eventually make enough tests to pass cargo-mutants (no priority yet).

# Development

unsynn is meant to evolve opportunistically. When you spot a problem or need a new feature
feel free to open an [issue](https://git.pipapo.org/cehteh/unsynn/issues) or send a
[PR](https://git.pipapo.org/cehteh/unsynn/pulls).
