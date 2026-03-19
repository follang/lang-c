# External Fixture Sources

This file records the upstream provenance for vendored external fixtures under
`test/full_apps/external/`.

## Rules

- Keep the original upstream project and path.
- Record an exact version, tag, or commit.
- Record the applicable license.
- Note any local transformation needed to make the fixture deterministic or
  usable in this repository.

## Fixtures

### musl stdint header set

- Project: musl
- Upstream: https://git.musl-libc.org/cgit/musl/
- Version: `v1.2.5`
- License: MIT
- Local license file: `test/full_apps/licenses/musl-MIT.txt`
- Source files:
  - `include/stdint.h`
  - `arch/x86_64/bits/stdint.h`
  - `arch/x86_64/bits/alltypes.h.in`
  - `include/alltypes.h.in`
  - `COPYRIGHT`
- Local transformation:
  - `bits/alltypes.h` is generated as a pinned local fixture from the official
    `alltypes.h.in` inputs by replacing musl's generation macros with ordinary C
    declarations.

### Candidate future fixtures

- zlib `v1.3.1`
  - https://github.com/madler/zlib
- libpng official repository
  - https://github.com/pnggroup/libpng
- SQLite amalgamation docs
  - https://www.sqlite.org/amalgamation.html
