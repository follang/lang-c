//! Source-level IR for the PARC frontend contract.
//!
//! This module defines the durable intermediate representation that PARC
//! produces from parsed C source. It is:
//!
//! - Smaller and more canonical than the parser AST
//! - Serializable via serde (JSON, etc.)
//! - Independent of parser node shapes
//! - Free of link/binary/ABI-proof concerns
//!
//! Downstream consumers (LINC, GERC) should depend only on these types,
//! never on `pac::ast` directly.

pub mod diagnostics;
pub mod items;
pub mod macros;
pub mod package;
pub mod provenance;
pub mod target;
pub mod types;

pub use diagnostics::{DiagnosticKind, Severity, SourceDiagnostic, SourceLocation};
pub use items::{
    CallingConvention, RecordKind, SourceEnum, SourceEnumVariant, SourceField, SourceFunction,
    SourceItem, SourceItemKind, SourceParameter, SourceRecord, SourceTypeAlias, SourceUnsupported,
    SourceVariable,
};
pub use macros::{MacroForm, MacroKind, MacroValue, SourceMacro};
pub use package::SourcePackage;
pub use provenance::{DeclarationProvenance, MacroProvenance, SourceOrigin};
pub use target::{SourceDefine, SourceInputs, SourceTarget};
pub use types::{SourceType, TypeQualifiers};
