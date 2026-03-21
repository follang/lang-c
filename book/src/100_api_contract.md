# API Contract

This chapter records the intended public consumer surface of `parc`.

It is not a blanket promise about every future change. It is the current explicit guidance for how
downstream tools should integrate with the crate without depending on internal parser details.

## First Principle

`parc` is a C language frontend: preprocessing, parsing, and source-level semantic extraction.

The intended downstream pattern is:

1. scan headers or parse source via `driver`, `scan`, or `parse`
2. extract normalized declarations via `extract`
3. consume the `SourcePackage` IR from `ir`
4. use `visit`, `span`, and `loc` to analyze AST-level details if needed

Downstream consumers should depend on `parc::ir`, not on `parc::ast` directly.

More importantly for this repository:

- `parc` library code must not depend on `linc` or `gec`
- `linc` and `gec` should not require `parc` as a library dependency in their
  production code paths
- integration should happen through PARC-owned artifacts in tests/examples or
  external harnesses
- there is no shared ABI crate that all three libraries depend on

## Preferred public surface

These are the main consumer-facing modules:

| Module | Role | Current expectation |
| --- | --- | --- |
| `parc::ir` | source-level IR (`SourcePackage`) | **preferred data contract** |
| `parc::extract` | declaration extraction from AST | preferred extraction entry point |
| `parc::scan` | header scanning (preprocess + extract) | preferred high-level entry point |
| `parc::intake` | preprocessed source intake | preferred for already-preprocessed source |
| `parc::driver` | parse files and preprocessed source | preferred parse entry point |
| `parc::preprocess` | built-in C preprocessor | preferred preprocessing entry point |
| `parc::parse` | parse string fragments directly | preferred low-level entry point |
| `parc::ast` | typed syntax tree | internal data model |
| `parc::visit` | recursive traversal hooks | preferred traversal API |
| `parc::span` | byte-range metadata | preferred location primitive |
| `parc::loc` | map offsets back to files/lines | preferred diagnostics helper |
| `parc::print` | AST debug dumping | preferred inspection helper |

## Internal modules are not the contract

These modules are public only indirectly through behavior, not as a recommended downstream surface:

- `parser`
- `env`
- `astutil`
- `strings`

If a downstream tool depends directly on how those modules work, it is probably coupling itself to
implementation details rather than the intended library boundary.

## Normative consumer rules

If you are building on top of `parc`, the safest current rules are:

1. use `driver` when preprocessing matters
2. use `parse::*` for fragment parsing or already-controlled text inputs
3. treat `ir::SourcePackage` as the primary output contract
4. use `visit` for traversal instead of hand-rolling recursive descent everywhere
5. use `span` and `loc` for diagnostics rather than guessing source positions
6. do not rely on exact error-message strings for durable control flow
7. do not treat PAC as semantic analysis, type checking, or ABI proof
8. if another crate needs PARC output, serialize the PARC-owned artifact and
   translate it outside library code

## What is part of the practical contract

Today the strongest practical contract is:

- `ir::SourcePackage`, `SourceType`, `SourceItem`, and all IR types — the primary data contract
- `extract::extract_from_source`, `extract_from_translation_unit`, `parse_and_extract`, `parse_and_extract_resilient`
- `scan::ScanConfig`, `scan_headers`, `ScanResult`
- `intake::PreprocessedInput`
- `ir::SourcePackageBuilder` — programmatic package construction
- `driver::Config`, `Flavor`, `Parse`, `Error`, `SyntaxError`, `parse_builtin`, and `capture_macros`
- `preprocess::{Processor, IncludeResolver, MacroTable, Lexer, preprocess, tokens_to_text, Target, define_target_macros}`
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

- semantic name resolution beyond parsing decisions such as typedef handling
- type checking
- ABI compatibility guarantees
- full support for every GCC or Clang extension
- preservation of raw macro definitions beyond what `capture_macros` provides

Those are outside the scope of PAC as a parser library.

## Downstream posture

For long-lived integrations, the safest posture is:

1. use `scan` or `extract` as your primary entry point — these produce `SourcePackage`
2. consume `ir::SourcePackage` rather than raw AST types where possible
3. use `driver` and `parse` only when you need AST-level access
4. treat unsupported syntax and parser errors as normal outcomes
5. keep tests with representative preprocessed inputs for the syntax families you depend on
6. see [Migration From bic](./210_migration_from_bic.md) if you are transitioning from `bic`
