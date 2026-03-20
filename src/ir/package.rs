//! Top-level source package for the PARC frontend contract.

use serde::{Deserialize, Serialize};

use super::diagnostics::SourceDiagnostic;
use super::items::{
    SourceEnum, SourceFunction, SourceItem, SourceItemKind, SourceRecord, SourceTypeAlias,
    SourceUnsupported, SourceVariable,
};
use super::macros::SourceMacro;
use super::provenance::{DeclarationProvenance, MacroProvenance};
use super::target::{SourceInputs, SourceTarget};

pub const SCHEMA_VERSION: u32 = 1;

fn default_schema_version() -> u32 {
    SCHEMA_VERSION
}

/// The top-level frontend contract produced by PARC.
///
/// Contains all source-level information extracted from C headers:
/// declarations, macros, diagnostics, and provenance. Independent of
/// any link/binary/ABI-proof concerns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourcePackage {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub target: SourceTarget,
    #[serde(default)]
    pub inputs: SourceInputs,
    #[serde(default)]
    pub items: Vec<SourceItem>,
    #[serde(default)]
    pub macros: Vec<SourceMacro>,
    #[serde(default)]
    pub diagnostics: Vec<SourceDiagnostic>,
    #[serde(default)]
    pub provenance: Vec<DeclarationProvenance>,
    #[serde(default)]
    pub macro_provenance: Vec<MacroProvenance>,
    #[serde(default)]
    pub source_path: Option<String>,
}

impl SourcePackage {
    pub fn new() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            target: SourceTarget::default(),
            inputs: SourceInputs::default(),
            items: Vec::new(),
            macros: Vec::new(),
            diagnostics: Vec::new(),
            provenance: Vec::new(),
            macro_provenance: Vec::new(),
            source_path: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.diagnostics.is_empty() && self.macros.is_empty()
    }

    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn functions(&self) -> impl Iterator<Item = &SourceFunction> {
        self.items.iter().filter_map(|item| match item {
            SourceItem::Function(f) => Some(f),
            _ => None,
        })
    }

    pub fn records(&self) -> impl Iterator<Item = &SourceRecord> {
        self.items.iter().filter_map(|item| match item {
            SourceItem::Record(r) => Some(r),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &SourceEnum> {
        self.items.iter().filter_map(|item| match item {
            SourceItem::Enum(e) => Some(e),
            _ => None,
        })
    }

    pub fn type_aliases(&self) -> impl Iterator<Item = &SourceTypeAlias> {
        self.items.iter().filter_map(|item| match item {
            SourceItem::TypeAlias(t) => Some(t),
            _ => None,
        })
    }

    pub fn variables(&self) -> impl Iterator<Item = &SourceVariable> {
        self.items.iter().filter_map(|item| match item {
            SourceItem::Variable(v) => Some(v),
            _ => None,
        })
    }

    pub fn unsupported_items(&self) -> impl Iterator<Item = &SourceUnsupported> {
        self.items.iter().filter_map(|item| match item {
            SourceItem::Unsupported(u) => Some(u),
            _ => None,
        })
    }

    pub fn find_function(&self, name: &str) -> Option<&SourceFunction> {
        self.functions().find(|f| f.name == name)
    }

    pub fn find_record(&self, name: &str) -> Option<&SourceRecord> {
        self.records().find(|r| r.name.as_deref() == Some(name))
    }

    pub fn find_enum(&self, name: &str) -> Option<&SourceEnum> {
        self.enums().find(|e| e.name.as_deref() == Some(name))
    }

    pub fn find_type_alias(&self, name: &str) -> Option<&SourceTypeAlias> {
        self.type_aliases().find(|t| t.name == name)
    }

    pub fn find_variable(&self, name: &str) -> Option<&SourceVariable> {
        self.variables().find(|v| v.name == name)
    }

    pub fn find_unsupported(&self, name: &str) -> Option<&SourceUnsupported> {
        self.unsupported_items()
            .find(|u| u.name.as_deref() == Some(name))
    }

    pub fn find_macro(&self, name: &str) -> Option<&SourceMacro> {
        self.macros.iter().find(|m| m.name == name)
    }

    pub fn count_by_kind(&self) -> std::collections::HashMap<SourceItemKind, usize> {
        let mut counts = std::collections::HashMap::new();
        for item in &self.items {
            *counts.entry(item.kind()).or_insert(0) += 1;
        }
        counts
    }

    pub fn function_count(&self) -> usize {
        self.functions().count()
    }

    pub fn record_count(&self) -> usize {
        self.records().count()
    }

    pub fn enum_count(&self) -> usize {
        self.enums().count()
    }

    pub fn type_alias_count(&self) -> usize {
        self.type_aliases().count()
    }

    pub fn variable_count(&self) -> usize {
        self.variables().count()
    }

    pub fn unsupported_count(&self) -> usize {
        self.unsupported_items().count()
    }
}

impl Default for SourcePackage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::diagnostics::{DiagnosticKind, Severity};
    use crate::ir::items::*;
    use crate::ir::macros::*;
    use crate::ir::types::SourceType;

    #[test]
    fn empty_package() {
        let pkg = SourcePackage::new();
        assert!(pkg.is_empty());
        assert!(!pkg.has_diagnostics());
        assert_eq!(pkg.item_count(), 0);
        assert_eq!(pkg.schema_version, SCHEMA_VERSION);
        assert!(pkg.source_path.is_none());
    }

    #[test]
    fn package_with_items() {
        let mut pkg = SourcePackage::new();
        pkg.items.push(SourceItem::Function(SourceFunction {
            name: "malloc".into(),
            calling_convention: CallingConvention::C,
            parameters: vec![SourceParameter {
                name: Some("size".into()),
                ty: SourceType::ULong,
            }],
            return_type: SourceType::ptr(SourceType::Void),
            variadic: false,
            source_offset: Some(10),
        }));
        pkg.items.push(SourceItem::Record(SourceRecord {
            kind: RecordKind::Struct,
            name: Some("point".into()),
            fields: Some(vec![SourceField {
                name: Some("x".into()),
                ty: SourceType::Int,
                bit_width: None,
            }]),
            source_offset: Some(20),
        }));
        pkg.items.push(SourceItem::Enum(SourceEnum {
            name: Some("color".into()),
            variants: vec![SourceEnumVariant {
                name: "RED".into(),
                value: Some(0),
            }],
            source_offset: None,
        }));
        pkg.items.push(SourceItem::TypeAlias(SourceTypeAlias {
            name: "size_t".into(),
            target: SourceType::ULong,
            source_offset: None,
        }));
        pkg.items.push(SourceItem::Variable(SourceVariable {
            name: "errno".into(),
            ty: SourceType::Int,
            source_offset: None,
        }));

        assert!(!pkg.is_empty());
        assert_eq!(pkg.item_count(), 5);
        assert!(pkg.find_function("malloc").is_some());
        assert!(pkg.find_record("point").is_some());
        assert!(pkg.find_enum("color").is_some());
        assert!(pkg.find_type_alias("size_t").is_some());
        assert!(pkg.find_variable("errno").is_some());
        assert!(pkg.find_function("nonexistent").is_none());

        let counts = pkg.count_by_kind();
        assert_eq!(counts[&SourceItemKind::Function], 1);
        assert_eq!(counts[&SourceItemKind::Record], 1);
    }

    #[test]
    fn package_with_diagnostics() {
        let mut pkg = SourcePackage::new();
        pkg.diagnostics.push(SourceDiagnostic {
            kind: DiagnosticKind::ParseFailed,
            severity: Severity::Error,
            message: "bad token".into(),
            location: None,
            item_name: None,
        });
        assert!(pkg.has_diagnostics());
        assert!(!pkg.is_empty());
    }

    #[test]
    fn package_with_macros() {
        let mut pkg = SourcePackage::new();
        pkg.macros.push(SourceMacro {
            name: "SIZE".into(),
            body: "42".into(),
            form: MacroForm::ObjectLike,
            kind: MacroKind::Integer,
            value: Some(MacroValue::Integer(42)),
        });
        assert!(!pkg.is_empty());
    }

    #[test]
    fn json_roundtrip_full_package() {
        let mut pkg = SourcePackage::new();
        pkg.source_path = Some("test.h".into());
        pkg.items.push(SourceItem::Function(SourceFunction {
            name: "foo".into(),
            calling_convention: CallingConvention::C,
            parameters: vec![],
            return_type: SourceType::Void,
            variadic: false,
            source_offset: None,
        }));
        pkg.macros.push(SourceMacro {
            name: "BAR".into(),
            body: "1".into(),
            form: MacroForm::ObjectLike,
            kind: MacroKind::Integer,
            value: Some(MacroValue::Integer(1)),
        });
        pkg.diagnostics.push(SourceDiagnostic::warning(
            DiagnosticKind::DeclarationPartial,
            "incomplete",
        ));

        let json = serde_json::to_string_pretty(&pkg).unwrap();
        let back: SourcePackage = serde_json::from_str(&json).unwrap();
        assert_eq!(pkg, back);
    }

    #[test]
    fn deserialize_minimal_json() {
        // Minimal valid JSON (only required fields absent, defaults fill in)
        let json = r#"{"items":[],"diagnostics":[]}"#;
        let pkg: SourcePackage = serde_json::from_str(json).unwrap();
        assert_eq!(pkg.schema_version, SCHEMA_VERSION);
        assert!(pkg.items.is_empty());
    }

    #[test]
    fn deterministic_serialization() {
        let mut pkg = SourcePackage::new();
        pkg.items.push(SourceItem::Variable(SourceVariable {
            name: "x".into(),
            ty: SourceType::Int,
            source_offset: None,
        }));
        let a = serde_json::to_string(&pkg).unwrap();
        let b = serde_json::to_string(&pkg).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn default_trait() {
        let pkg = SourcePackage::default();
        assert!(pkg.is_empty());
    }
}
