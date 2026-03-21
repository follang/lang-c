# PARC

[![Documentation](https://docs.rs/parc/badge.svg)](https://docs.rs/parc)

PARC is a C language frontend: preprocessing, parsing, header scanning, and
source-level semantic extraction.

In the intended repository architecture, `parc` owns source meaning only.

- `parc` library code does not depend on `linc` or `gec`
- `parc` emits a PARC-owned source artifact
- downstream crates may consume that artifact in tests, examples, or external
  harnesses
- there is no shared ABI crate and no library-level cross-package adapter

```rust
use parc::driver::{Config, parse};

fn main() {
    let config = Config::default();
    println!("{:?}", parse(&config, "example.c"));
}
```

# Bugs

Just open an issue, bug reports and patches are most welcome.

## License

Dual-licenced under Apache 2.0 or MIT licenses (see `LICENSE-APACHE` and `LICENSE-MIT` for legal terms).

## Development

## Artifact Boundary

`parc` is not the owner of a universal pipeline type.

It owns its own source model and its own serialized artifact story.
That artifact should contain:

- extracted declarations
- normalized source types
- macros that matter downstream
- provenance
- diagnostics
- partial or unsupported declarations

Cross-package integration belongs outside `parc` library code. If `linc` or
`gec` wants to consume `parc` output, that translation should happen in:

- `linc/tests/**`
- `linc/examples/**`
- `gec/tests/**`
- `gec/examples/**`
- external integration harnesses

Never in `parc/src/**`.

The parser implementation lives under `src/parser/` and is maintained directly in Rust.
There is no external PEG generation step in the build anymore.

The reference fixtures live under `test/reftests/`.
Larger translation-unit and multi-file fixtures live under `test/full_apps/`.

The makefile mirrors the normal Cargo flow:

- `make build` builds the library
- `make test` runs all tests

### Full app fixtures

Each full-app fixture lives in its own directory under `test/full_apps/` and includes a
`fixture.toml` manifest plus one or more source files.

Supported manifest fields:

- `name` identifies the fixture in a human-readable way
- `mode` is one of `translation_unit`, `driver`, or `preprocessed`
- `flavor` is one of `std`, `gnu`, or `clang`
- `entry` points at the file loaded by the harness
- `expected` is `parse_ok` or `parse_error`
- `include_dirs` lists local include roots for `driver` mode fixtures
- `tags` adds corpus filters such as `synthetic`, `external`, or `preprocessed`
- `source`, `upstream_ref`, `license`, and `notes` record provenance

Example:

```toml
name = "mini_http"
mode = "driver"
flavor = "gnu"
entry = "src/main.c"
expected = "parse_ok"
include_dirs = ["include"]
tags = ["synthetic", "multi_file"]
source = "synthetic"
license = "repo"
```

Mode behavior:

- `translation_unit` parses the entry file directly
- `driver` preprocesses through the configured C compiler and then parses
- `preprocessed` parses a pinned `.i` snapshot through `driver::parse_preprocessed`

### Test filtering and CI tiers

The full-app runner supports these environment filters:

- `FULL_APP_FILTER=<path-fragment>` runs only matching fixture directories
- `FULL_APP_TAG=<tag>` runs only fixtures whose manifest contains that tag
- `TEST_FILTER=<prefix>` still filters the legacy `test/reftests/` corpus

The CI workflow is split into two tiers:

- fast tier: push and pull request coverage for unit tests, reftests, and synthetic full apps
- slow tier: scheduled coverage for the complete full-app corpus, including external fixtures

To reproduce the fast CI tier locally:

```sh
FULL_APP_TAG=synthetic make test
```

To run one full-app fixture locally:

```sh
FULL_APP_FILTER=musl/stdint make test
```

### Adding external fixtures

External fixtures must be pinned and documented. When adding one:

1. Create a new directory under `test/full_apps/external/`.
2. Add a `fixture.toml` with `source`, `upstream_ref`, `license`, `notes`, and `tags`.
3. Record the provenance in `test/full_apps/EXTERNAL_SOURCES.md`.
4. Add the corresponding license text or note under `test/full_apps/licenses/`.
5. Keep the imported files deterministic and trimmed only when necessary.

The helper script:

```sh
test/full_apps/scripts/refresh_external_fixture.sh list
test/full_apps/scripts/refresh_external_fixture.sh show musl-stdint
```

documents the pinned external fixtures already in the repository. When an external fixture is
re-pinned, update the vendored files, `fixture.toml`, `EXTERNAL_SOURCES.md`, and the
corresponding license notes in the same change.
