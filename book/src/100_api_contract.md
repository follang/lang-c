# API Contract

This chapter records the intended public consumer surface of `pac`.

It is not a blanket promise about every future change. It is the current explicit guidance for how
downstream tools should integrate with the crate without depending on internal parser details.

## First Principle

`pac` is a parsing library.

The intended downstream pattern is:

1. parse source with `driver` or `parse`
2. consume typed AST values from `ast`
3. use `visit`, `span`, and `loc` to analyze and report on those values

Consumers should treat parser internals as implementation, not as the main integration surface.

## Preferred public surface

These are the main consumer-facing modules:

| Module | Role | Current expectation |
| --- | --- | --- |
| `pac::driver` | parse files and preprocessed source | preferred high-level entry point |
| `pac::parse` | parse string fragments directly | preferred low-level entry point |
| `pac::ast` | typed syntax tree | preferred data contract |
| `pac::visit` | recursive traversal hooks | preferred traversal API |
| `pac::span` | byte-range metadata | preferred location primitive |
| `pac::loc` | map offsets back to files/lines | preferred diagnostics helper |
| `pac::print` | AST debug dumping | preferred inspection helper |

## Internal modules are not the contract

These modules are public only indirectly through behavior, not as a recommended downstream surface:

- `parser`
- `env`
- `astutil`
- `strings`

If a downstream tool depends directly on how those modules work, it is probably coupling itself to
implementation details rather than the intended library boundary.

## Normative consumer rules

If you are building on top of `pac`, the safest current rules are:

1. use `driver` when preprocessing matters
2. use `parse::*` for fragment parsing or already-controlled text inputs
3. treat `ast` types as the primary output contract
4. use `visit` for traversal instead of hand-rolling recursive descent everywhere
5. use `span` and `loc` for diagnostics rather than guessing source positions
6. do not rely on exact error-message strings for durable control flow
7. do not treat PAC as semantic analysis, type checking, or ABI proof

## What is part of the practical contract

Today the strongest practical contract is:

- `driver::Config`, `Flavor`, `Parse`, `Error`, and `SyntaxError`
- `parse::{constant, expression, declaration, statement, translation_unit, translation_unit_resilient}`
- the AST model under `ast`
- the traversal hooks under `visit`
- the span/location model under `span` and `loc`

Those are the surfaces the rest of the book assumes consumers will use.

## What is intentionally weaker

The following should be treated as less stable than the core parsing surface:

- exact debug formatting of AST values
- exact `Display` wording of parse errors
- internal parser file layout under `src/parser/`
- incidental ordering of implementation helper functions

These details are useful for debugging and contribution work, but they are not the main consumer
contract.

## Explicit non-goals

The current contract does not promise:

- preprocessing without a system C compiler in the `driver` path
- semantic name resolution beyond parsing decisions such as typedef handling
- type checking
- ABI compatibility guarantees
- full support for every GCC or Clang extension
- preservation of raw macro definitions as a first-class PAC output

Those are outside the scope of PAC as a parser library.

## Downstream posture

For long-lived integrations, the safest posture is:

1. keep your entry points at `driver` and `parse`
2. convert ASTs into your own analysis model if you need stricter invariants
3. treat unsupported syntax and parser errors as normal outcomes
4. keep tests with representative preprocessed inputs for the syntax families you depend on
