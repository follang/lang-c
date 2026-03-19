# Full App Fixtures

This directory holds larger C programs than the micro fixtures in `test/reftests/`.

Each fixture lives in its own directory and includes a `fixture.toml` manifest plus one or
more C source or header files.

Current scope:

- synthetic single-file apps
- synthetic multi-file apps
- curated external fixtures added later with pinned upstream metadata

The first fixture here is intentionally small. It establishes the test layout and runner
before the corpus grows into dozens of cases.
