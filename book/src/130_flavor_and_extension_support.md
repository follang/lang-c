# Flavor And Extension Support

PAC supports three language flavors and several extension families.

This chapter records what that means in practice.

## Flavors

| Flavor | Intent |
| --- | --- |
| `StdC11` | strict C11 parsing |
| `GnuC11` | C11 plus GNU-oriented syntax |
| `ClangC11` | C11 plus Clang-oriented syntax |

Use the flavor that matches the syntax you expect in the input.

## Why flavor matters

Some C parses are ambiguous or extension-specific.

Examples include:

- GNU statement expressions
- `typeof`
- GCC-style attributes
- GNU asm statements
- Clang availability attributes

If you parse extension-heavy source in `StdC11`, errors are expected.

## GNU-oriented support

The AST and parser explicitly model GNU-oriented syntax such as:

- `typeof`
- statement expressions
- GNU asm statements
- asm labels
- attributes
- designated range initializers

In practice, if the source is GCC-flavored or Linux-kernel-like, `GnuC11` is usually the right
starting point.

## Clang-oriented support

PAC also models Clang-specific or Clang-common syntax including:

- Clang availability attributes
- the `ClangC11` flavor path in `driver` and `parse`

If your preprocessing and syntax assumptions are built around Clang, use `Config::with_clang()` or
`Flavor::ClangC11`.

## C23 keyword support

PAC accepts the following C23 keywords in all flavors, because modern compilers (GCC 15+) emit them
in preprocessed output by default:

| C23 keyword | C11 equivalent | Notes |
| --- | --- | --- |
| `bool` | `_Bool` | type specifier |
| `true` | `1` | parsed as integer constant |
| `false` | `0` | parsed as integer constant |
| `nullptr` | `0` | parsed as integer constant |
| `static_assert` | `_Static_assert` | declaration |
| `alignas` | `_Alignas` | alignment specifier |
| `alignof` | `_Alignof` | expression |
| `thread_local` | `_Thread_local` | storage class |
| `constexpr` | (none) | storage class specifier |
| `typeof` | `__typeof__` | type specifier (was GNU-only) |
| `_BitInt(N)` | (none) | type specifier with width |
| `noreturn` | `_Noreturn` | function specifier |
| `complex` | `_Complex` | type specifier |

## GCC extension types

PAC recognizes these GCC extension types in GNU mode:

| Type | AST variant | Notes |
| --- | --- | --- |
| `__int128` | `TypeSpecifier::Int128` | non-unique, combinable with `signed`/`unsigned` |
| `__float128` | `TypeSpecifier::Float128` | unique type specifier |
| `__builtin_va_list` | typedef | handled as built-in typedef name |

## Standard-mode guidance

Use `StdC11` when:

- you want to reject vendor syntax deliberately
- your test corpus is intended to stay close to the standard
- you want parser behavior that is easier to reason about across compilers

## Practical consumer policy

A useful integration policy is:

1. default to the compiler family you actually preprocess with
2. add tests for the specific extension families you rely on
3. treat unsupported extensions as explicit parser limitations, not random bugs

## What this chapter does not claim

This chapter does not claim exhaustive support for every extension accepted by GCC or Clang.

It does claim that PAC has explicit support for several important extension families and that the
flavor setting is part of the API contract for using them correctly.
