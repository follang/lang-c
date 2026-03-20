//! Built-in C preprocessor
//!
//! Provides tokenization, directive parsing, macro expansion, and
//! conditional compilation without requiring an external `gcc -E` invocation.

mod token;
mod lexer;
mod directive;
mod macros;

pub use self::token::{Token, TokenKind};
pub use self::lexer::Lexer;
pub use self::directive::{Directive, parse_directive};
pub use self::macros::{MacroDef, MacroTable};
