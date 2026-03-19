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

### zlib public header set

- Project: zlib
- Upstream: https://github.com/madler/zlib
- Version: `v1.3.1`
- License: Zlib
- Local license file: `test/full_apps/licenses/zlib-License.txt`
- Source files:
  - `zlib.h`
  - `zconf.h`
  - `LICENSE`
- Local transformation:
  - Added a local `main.c` wrapper translation unit that includes `zlib.h` and
    exercises representative public API types without modifying the vendored
    headers.

### zlib zpipe example

- Project: zlib
- Upstream: https://github.com/madler/zlib
- Version: `v1.3.1`
- License: Public domain
- Source files:
  - `examples/zpipe.c`
  - `zlib.h`
  - `zconf.h`
- Local transformation:
  - Trimmed the example down to a parser-oriented subset while preserving the
    original declarations, includes, and API usage pattern.
  - The local fixture reuses the vendored zlib public headers and opts into host
    libc headers explicitly via the manifest.

### Candidate future fixtures

- zlib `v1.3.1`
  - https://github.com/madler/zlib
- libpng official repository
  - https://github.com/pnggroup/libpng
- SQLite amalgamation docs
  - https://www.sqlite.org/amalgamation.html
