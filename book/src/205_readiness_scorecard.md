# Readiness Scorecard

This chapter ties PARC readiness to real suites instead of vague confidence
claims.

## Overall Posture

PARC should currently be read as:

- strong on parser and extraction fundamentals
- strong on scan-first vendored baselines
- materially stronger on hostile real-world builtin-preprocessor corners
- intentionally conservative when a large header family cannot be modeled
  honestly

For Level 1 production, PARC should be read as Linux/ELF-first and
canonical-corpus-backed, not as a universal frontend for arbitrary C headers.

For whole-pipeline claims, this score is also capped by downstream `gerc`
anchors that ingest translated PARC source surfaces in tests/examples.

That is good progress, but it is not the same thing as "finished for every C
header in the wild".

## Subsystem Scorecard

- parser entrypoints: high
- AST traversal and printing: high
- extraction to `SourcePackage`: high
- scan-first vendored baselines: high
- hostile-header recovery: medium-high
- built-in preprocessor coverage on ugly system headers: medium-high
- large host-dependent wrapper extraction: medium-high
- deterministic behavior on canonical large surfaces: high

## Canonical Readiness Anchors

The release posture should be judged against these anchors first:

- vendored musl `stdint`
- vendored zlib
- vendored libpng scan
- repo-owned `macro_env_a`
- repo-owned `type_env_b`
- OpenSSL public wrapper extraction
- combined Linux event-loop wrapper extraction

If those anchors stay green and deterministic, PARC is earning trust. If they
drift, the scorecard should be lowered even if many smaller tests still pass.

## What Would Raise Readiness Further

The next meaningful gains would be:

- broader built-in-preprocessor coverage on other hostile width and platform
  gates beyond the libpng family
- more ugly combined system-header clusters
- more repeat-run deterministic scans on large host-dependent surfaces
- clearer unsupported-case diagnostics for the remaining difficult families
