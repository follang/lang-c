# PARC Reference

PARC is a Rust library for C language frontend processing: preprocessing,
parsing, and source-level semantic extraction. It targets C11 and can
optionally accept GNU and Clang extensions.

PARC produces a durable `SourcePackage` contract suitable for downstream
consumption by linker and codegen stages.

## What PARC Owns

- **Preprocessing**: built-in C preprocessor with macro expansion, conditionals, and includes
- **Parsing**: C11 parser producing a typed AST under `parc::ast`
- **Extraction**: source-level declaration normalization into `parc::ir`
- **Source IR**: a serializable `SourcePackage` with functions, records, enums, typedefs, variables, macros, diagnostics, and provenance
- **Scanning**: end-to-end header scanning via `parc::scan`
- Source spans under `parc::span`
- File/line reconstruction under `parc::loc`
- Recursive visitor API under `parc::visit`
- Tree-style debug printer under `parc::print`

## What PARC Does Not Own

- native symbol inspection
- binary validation
- link-plan construction
- Rust lowering or emission

## Data flow

```text
C source / headers
  -> preprocessor (built-in or gcc/clang)
  -> PARC parser
  -> extraction
  -> SourcePackage (frontend contract)
  -> downstream (LINC, GERC)
```

## Module layout

| Module | Purpose |
| --- | --- |
| `ir` | Source-level IR: `SourcePackage`, `SourceType`, `SourceItem`, etc. |
| `extract` | Declaration extraction from AST to IR |
| `scan` | Header scanning (preprocess + parse + extract) |
| `intake` | Preprocessed source intake |
| `driver` | High-level API for parsing files via a C preprocessor |
| `preprocess` | Built-in C preprocessor |
| `parse` | Direct parsing of expressions, declarations, statements, translation units |
| `ast` | AST definitions for declarations, expressions, statements, extensions |
| `visit` | Recursive traversal API for AST consumers |
| `span` | Byte offsets for parsed nodes |
| `loc` | Mapping byte offsets back to source files and lines |
| `print` | Debug-oriented tree printer for ASTs |

## Artifact boundary

`parc` owns source meaning only.

The `SourcePackage` boundary is where `parc` stops:

- `parc/src/**` must not depend on `linc` or `gec`
- downstream translation belongs only in tests, examples, or external harnesses
- `parc` does not own link or generation concerns

## Typical use cases

- Extract function/struct/enum declarations from C headers
- Build a linter or analysis tool for C code
- Scan headers and produce a serializable source package
- Parse small fragments in tests
- Prototype refactoring or code-search tools

## Where to start

- Read [Getting Started](./010_getting_started.md) for basic setup
- Read [Source IR](./026_source_ir.md) for the data contract
- Read [Extraction](./027_extraction.md) for turning source into IR
- Read [Header Scanning](./028_scanning.md) for end-to-end workflows
- Read [Driver API](./020_driver.md) or [Parser API](./030_parser.md) for parse-level access
- Read [AST Model](./040_ast.md) before writing analysis code
