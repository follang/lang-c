# PARC

[![Documentation](https://docs.rs/parc/badge.svg)](https://docs.rs/parc)

PARC is the source frontend of the toolchain. It owns preprocessing, parsing,
header scanning, source extraction, source diagnostics, and the PARC-owned
source IR.

## Scoped Production Statement

The current Level 1 production claim for the whole pipeline is:

- Linux/ELF-first
- canonical-corpus-backed
- conservative on unsupported source families

For PARC specifically, that means:

- production-ready as the source frontend for the documented canonical corpus
- not a claim of universal C frontend parity
- not a claim that every hostile extension family is fully supported

## Level 1 Support Matrix

| Area | Level 1 status | Notes |
|---|---|---|
| Linux/ELF-oriented public headers | primary production scope | This is the first production envelope. |
| Apple-oriented public headers | secondary confidence scope | Useful and tested, but not the primary production claim. |
| Windows-oriented public headers | secondary confidence scope | Not the first production envelope. |
| K&R declarations | diagnostics-only | Explicit unsupported diagnostics are part of the contract. |
| block pointers | rejected | Explicit rejection is part of the contract. |
| bitfield-heavy records | partial | Source shape is preserved, full layout truth is not promised. |
| vendor attributes | partial | Declarations are preserved with partial diagnostics where ignored. |
| macro-heavy hostile stacks | corpus-backed support | Supported on the named canonical corpora, not claimed universally. |

The current crate surface is broader than “just a parser”:

- `driver` parses files through an external preprocessor
- `preprocess` provides a built-in C preprocessor
- `parse` parses source fragments directly
- `extract` lowers AST into source IR
- `scan` turns real headers into a `SourcePackage`
- `ir` is the durable source contract

## Ownership

PARC owns:

- C preprocessing and preprocessing-related capture helpers
- C parsing and parser recovery
- AST traversal, spans, locations, and debug printing
- source-level extraction into `parc::ir::SourcePackage`
- end-to-end header scanning via `parc::scan`

PARC does not own:

- symbol inspection
- binary validation
- link planning
- Rust lowering or emission

## Real Public Surface

The most important public entrypoints today are:

- `parc::driver::{Config, parse, parse_preprocessed, parse_builtin, capture_macros}`
- `parc::scan::{ScanConfig, scan_headers}`
- `parc::extract::{Extractor, extract_from_source, parse_and_extract, parse_and_extract_resilient}`
- `parc::parse::*` for fragment parsing
- `parc::ir::*` for the source contract
- `parc::visit`, `parc::span`, `parc::loc`, and `parc::print`

The crate root is intentionally broad because PARC still serves both:

- downstream consumers that only want `SourcePackage`
- parser/AST-level consumers that want direct syntax access

## Fastest Working Paths

Parse a file through the normal external-preprocessor path:

```rust
use parc::driver::{parse, Config};

let parsed = parse(&Config::default(), "src/tests/files/minimal.c").unwrap();
println!("top-level items: {}", parsed.unit.0.len());
```

Scan headers and produce source IR directly:

```rust
use parc::scan::{scan_headers, ScanConfig};

let config = ScanConfig::new().entry_header("demo.h");
let result = scan_headers(&config).unwrap();
println!("ir items: {}", result.package.items.len());
```

Parse a fragment from memory:

```rust
use parc::driver::Flavor;
use parc::parse;

let expr = parse::expression("a + b * 2", Flavor::StdC11).unwrap();
println!("{expr:#?}");
```

## Artifact Boundary

`parc` owns its own source model and serialized source artifacts.

The durable boundary is `parc::ir::SourcePackage`, which contains:

- extracted items
- source types
- macros and input metadata
- provenance and diagnostics
- partial/unsupported source results

Cross-package translation still belongs outside `parc/src/**`. PARC can be
used in integration tests and harnesses, but its library code is not where
downstream link or generation wiring should live.

## Tested Scope

The current suite covers:

- parser and preprocessor behavior
- scan/extract/source-contract behavior
- determinism and JSON/source-artifact roundtrips
- hostile headers, system headers, and full-app fixtures
- explicit preprocess and source failure matrices
- external-fixture corpora under `src/tests/**`

The tests are the best statement of what PARC actually supports.

## Level 1 Contract

For the Level 1 production claim, the PARC contract is:

- supported families are supported on the named canonical corpus
- partial families emit explicit partial diagnostics
- diagnostics-only families preserve useful source surface where possible
- rejected families fail explicitly rather than degrading silently

This is a bounded frontend contract, not a universal C compatibility claim.

## Hardening Matrix

The current hardening ladder is easiest to read in four buckets:

- hermetic vendored baselines
  - musl `stdint`
  - vendored zlib
  - vendored libpng builtin-preprocessor success surface
- host-dependent public-header ladders
  - OpenSSL public wrapper extraction
  - libcurl public wrapper extraction
  - Linux combined event-loop wrapper extraction
- hostile and degraded surfaces
  - hostile declaration fixtures
  - repo-owned `macro_env_a` hostile macro corpus
  - repo-owned `type_env_b` hostile type corpus
  - explicit unsupported-family closure ledger for K&R, block pointers,
    bitfield-heavy records, and vendor attributes
  - resilient recovery fixtures
  - explicit preprocess failure matrix
  - explicit source refusal and recovery matrix
  - extraction-status summaries that distinguish supported, partial, and unsupported surfaces
- determinism anchors
  - vendored musl scan
  - vendored zlib scan
  - vendored libpng scan
  - `macro_env_a` scan
  - `type_env_b` scan
  - OpenSSL wrapper extraction
  - libcurl wrapper extraction
  - combined Linux event-loop wrapper extraction

Read those as the current confidence anchors, not as a promise that every
system header family is equally mature.

## Release Gates

`parc` should only be treated as release-ready when all of these remain green:

- `make build`
- `make test`
- hostile/system/full-app suites
- at least one vendored large-header scan target
- at least one combined system-header target
- deterministic repeated extraction on canonical large surfaces
- deterministic repeated extraction on repo-owned hostile corpora
- for whole-pipeline production claims, confirm the current downstream `gerc`
  canonical anchors still ingest `parc`-driven source surfaces in
  tests/examples

The Level 1 production floor is the hermetic subset of those gates:

- vendored musl `stdint`
- vendored zlib scan
- vendored libpng scan
- repo-owned `macro_env_a` hostile macro corpus
- repo-owned `type_env_b` hostile type corpus

The host-dependent confidence-raise layer is:

- OpenSSL public wrapper extraction
- libcurl public wrapper extraction
- combined Linux event-loop wrapper extraction

The current canonical frontend surfaces are:

- vendored musl `stdint`
- vendored zlib scan
- vendored libpng scan
- repo-owned `macro_env_a` hostile macro corpus
- repo-owned `type_env_b` hostile type corpus
- OpenSSL public wrapper extraction
- libcurl public wrapper extraction
- combined Linux event-loop wrapper extraction

## Canonical Corpus

The current PARC production corpus is intentionally named:

- hermetic vendored
  - musl `stdint`
  - zlib public headers
  - libpng public headers
- hermetic synthetic hostile
  - `test/corpus/macro_env_a`
  - `test/corpus/type_env_b`
- host-dependent raises
  - OpenSSL public wrapper extraction
  - libcurl public wrapper extraction
  - combined Linux event-loop wrapper extraction
- conservative-failure anchors
  - vendored zlib builtin-preprocessor conservative parse failure
  - malformed-source hard errors
  - resilient-source recovery paths

Those are the surfaces PARC should be judged against first.

Whole-pipeline readiness is stricter than crate-local readiness: downstream
`gerc` canonical anchors are part of the final production bar even though
`parc/src/**` must not depend on downstream crates.

## Build And Test

```sh
make build
make test
```

## Development Notes

The parser implementation lives under `src/parser/`.

The main source-contract and integration fixtures live under:

- `src/tests/`
- `src/tests/full_apps.rs`
- `src/tests/system_headers.rs`
- `src/tests/hostile_headers.rs`

The book is intentionally more detailed than this README. Start there if you
need the exact contract story for `driver`, `scan`, `extract`, or `ir`.

## License

Dual-licensed under Apache 2.0 or MIT (see `LICENSE-APACHE` and
`LICENSE-MIT`).
