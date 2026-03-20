# Error Surface

This chapter describes the error model PAC exposes today.

## Two layers of errors

PAC has two main error surfaces:

1. direct parser errors from `parse`
2. driver errors from `driver`

The distinction is important because the driver includes external preprocessing.

## Direct parser errors

The `parse` module returns:

```rust
Result<T, pac::parse::ParseError>
```

`ParseError` includes:

- `line`
- `column`
- `offset`
- `expected`

This error means:

- the parser could not consume the full input
- the failure happened at the given position
- one of the listed tokens or grammar expectations would have allowed parsing to continue

## Driver errors

The `driver` module returns:

```rust
Result<pac::driver::Parse, pac::driver::Error>
```

That error enum has two branches:

- `PreprocessorError(io::Error)`
- `SyntaxError(SyntaxError)`

This split is a real contract boundary:

- preprocessor failures mean PAC never reached parsing
- syntax failures mean preprocessing succeeded and PAC failed on the resulting text

## `SyntaxError`

`driver::SyntaxError` contains:

- `source`
- `line`
- `column`
- `offset`
- `expected`

It also provides:

- `get_location()` to map back to source files and include stack
- `format_expected()` for user-facing token formatting

## What consumers should key on

For durable control flow, consumers should branch on:

- error type
- structured fields such as `line`, `column`, and `expected`

Consumers should not branch on:

- exact human-readable `Display` text
- incidental token ordering inside formatted strings

## Practical examples

### Fragment parsing

```rust
use pac::driver::Flavor;
use pac::parse;

match parse::statement("if (x) {", Flavor::StdC11) {
    Ok(_) => {}
    Err(err) => {
        eprintln!("statement parse failed at {}:{}", err.line, err.column);
    }
}
```

### File parsing

```rust
use pac::driver::{parse, Config, Error};

match parse(&Config::default(), "broken.c") {
    Ok(_) => {}
    Err(Error::PreprocessorError(err)) => {
        eprintln!("preprocessor failure: {}", err);
    }
    Err(Error::SyntaxError(err)) => {
        let (loc, includes) = err.get_location();
        eprintln!("syntax failure in {}:{} ({})", loc.file, loc.line, err.column);
        eprintln!("include depth: {}", includes.len());
    }
}
```

## Resilient parsing

`parse::translation_unit_resilient` provides error recovery. When a declaration fails to parse, it
skips to the next synchronization point (`;` at file scope or `}` at brace depth zero) and continues
parsing.

```rust
use pac::driver::Flavor;
use pac::parse;

let tu = parse::translation_unit_resilient(source, Flavor::GnuC11);
// tu.0 contains all successfully parsed declarations
// unparseable regions are silently skipped
```

Use this when you want partial results from files that contain unsupported syntax. The strict
`translation_unit` function is still preferred when you need to detect all errors.

## Failure-model guidance

Downstream tools should treat parse failures as normal, reportable outcomes.

That means:

- do not crash just because one translation unit fails
- surface the structured error data to the caller
- retain the preprocessed source when debugging hard failures

## Explicit limitations of the current error model

The current model does not provide:

- semantic diagnostics
- fix-it suggestions
- a typed taxonomy for every grammar category of failure
- warning channels separate from parse success

PAC’s errors are syntax-oriented rather than compiler-like.
