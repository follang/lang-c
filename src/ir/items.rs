//! Source-level declaration items for the PARC frontend contract.

use serde::{Deserialize, Serialize};

use super::types::SourceType;

/// One extracted source-level declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SourceItem {
    Function(SourceFunction),
    Record(SourceRecord),
    Enum(SourceEnum),
    TypeAlias(SourceTypeAlias),
    Variable(SourceVariable),
    Unsupported(SourceUnsupported),
}

/// Discriminant tag for item classification without payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceItemKind {
    Function,
    Record,
    Enum,
    TypeAlias,
    Variable,
    Unsupported,
}

impl SourceItem {
    pub fn kind(&self) -> SourceItemKind {
        match self {
            SourceItem::Function(_) => SourceItemKind::Function,
            SourceItem::Record(_) => SourceItemKind::Record,
            SourceItem::Enum(_) => SourceItemKind::Enum,
            SourceItem::TypeAlias(_) => SourceItemKind::TypeAlias,
            SourceItem::Variable(_) => SourceItemKind::Variable,
            SourceItem::Unsupported(_) => SourceItemKind::Unsupported,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            SourceItem::Function(f) => Some(&f.name),
            SourceItem::Record(r) => r.name.as_deref(),
            SourceItem::Enum(e) => e.name.as_deref(),
            SourceItem::TypeAlias(t) => Some(&t.name),
            SourceItem::Variable(v) => Some(&v.name),
            SourceItem::Unsupported(u) => u.name.as_deref(),
        }
    }

    pub fn source_offset(&self) -> Option<usize> {
        match self {
            SourceItem::Function(f) => f.source_offset,
            SourceItem::Record(r) => r.source_offset,
            SourceItem::Enum(e) => e.source_offset,
            SourceItem::TypeAlias(t) => t.source_offset,
            SourceItem::Variable(v) => v.source_offset,
            SourceItem::Unsupported(u) => u.source_offset,
        }
    }
}

/// Calling convention annotation from source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallingConvention {
    C,
    Cdecl,
    Stdcall,
    Fastcall,
    Vectorcall,
    Thiscall,
    Unknown(String),
}

/// Extracted function declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceFunction {
    pub name: String,
    pub calling_convention: CallingConvention,
    pub parameters: Vec<SourceParameter>,
    pub return_type: SourceType,
    pub variadic: bool,
    pub source_offset: Option<usize>,
}

/// One function parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceParameter {
    pub name: Option<String>,
    pub ty: SourceType,
}

/// Kind of record (struct vs union).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordKind {
    Struct,
    Union,
}

/// One field inside a record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceField {
    pub name: Option<String>,
    pub ty: SourceType,
    #[serde(default)]
    pub bit_width: Option<u64>,
}

impl SourceField {
    pub fn is_bitfield(&self) -> bool {
        self.bit_width.is_some()
    }
}

/// Extracted record (struct/union) declaration.
///
/// `fields == None` means the record is opaque or incomplete.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRecord {
    pub kind: RecordKind,
    pub name: Option<String>,
    pub fields: Option<Vec<SourceField>>,
    pub source_offset: Option<usize>,
}

impl SourceRecord {
    pub fn is_opaque(&self) -> bool {
        self.fields.is_none()
    }
}

/// One enum constant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceEnumVariant {
    pub name: String,
    pub value: Option<i128>,
}

/// Extracted enum declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceEnum {
    pub name: Option<String>,
    pub variants: Vec<SourceEnumVariant>,
    pub source_offset: Option<usize>,
}

/// Extracted typedef/alias declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceTypeAlias {
    pub name: String,
    pub target: SourceType,
    pub source_offset: Option<usize>,
}

/// Extracted external variable declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceVariable {
    pub name: String,
    pub ty: SourceType,
    pub source_offset: Option<usize>,
}

/// Placeholder for a declaration the extractor could not model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceUnsupported {
    pub name: Option<String>,
    pub reason: String,
    pub source_offset: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_function() -> SourceFunction {
        SourceFunction {
            name: "malloc".into(),
            calling_convention: CallingConvention::C,
            parameters: vec![SourceParameter {
                name: Some("size".into()),
                ty: SourceType::ULong,
            }],
            return_type: SourceType::ptr(SourceType::Void),
            variadic: false,
            source_offset: Some(10),
        }
    }

    fn sample_record() -> SourceRecord {
        SourceRecord {
            kind: RecordKind::Struct,
            name: Some("point".into()),
            fields: Some(vec![
                SourceField {
                    name: Some("x".into()),
                    ty: SourceType::Int,
                    bit_width: None,
                },
                SourceField {
                    name: Some("y".into()),
                    ty: SourceType::Int,
                    bit_width: None,
                },
            ]),
            source_offset: Some(20),
        }
    }

    fn sample_enum() -> SourceEnum {
        SourceEnum {
            name: Some("color".into()),
            variants: vec![
                SourceEnumVariant {
                    name: "RED".into(),
                    value: Some(0),
                },
                SourceEnumVariant {
                    name: "GREEN".into(),
                    value: Some(1),
                },
                SourceEnumVariant {
                    name: "BLUE".into(),
                    value: Some(2),
                },
            ],
            source_offset: Some(30),
        }
    }

    #[test]
    fn source_item_kind() {
        let f = SourceItem::Function(sample_function());
        assert_eq!(f.kind(), SourceItemKind::Function);

        let r = SourceItem::Record(sample_record());
        assert_eq!(r.kind(), SourceItemKind::Record);

        let e = SourceItem::Enum(sample_enum());
        assert_eq!(e.kind(), SourceItemKind::Enum);

        let t = SourceItem::TypeAlias(SourceTypeAlias {
            name: "size_t".into(),
            target: SourceType::ULong,
            source_offset: None,
        });
        assert_eq!(t.kind(), SourceItemKind::TypeAlias);

        let v = SourceItem::Variable(SourceVariable {
            name: "errno".into(),
            ty: SourceType::Int,
            source_offset: None,
        });
        assert_eq!(v.kind(), SourceItemKind::Variable);

        let u = SourceItem::Unsupported(SourceUnsupported {
            name: Some("flags".into()),
            reason: "bitfield".into(),
            source_offset: None,
        });
        assert_eq!(u.kind(), SourceItemKind::Unsupported);
    }

    #[test]
    fn source_item_name() {
        assert_eq!(
            SourceItem::Function(sample_function()).name(),
            Some("malloc")
        );
        assert_eq!(
            SourceItem::Record(sample_record()).name(),
            Some("point")
        );
        assert_eq!(
            SourceItem::Enum(sample_enum()).name(),
            Some("color")
        );

        let anon_record = SourceItem::Record(SourceRecord {
            kind: RecordKind::Struct,
            name: None,
            fields: None,
            source_offset: None,
        });
        assert_eq!(anon_record.name(), None);
    }

    #[test]
    fn source_item_offset() {
        assert_eq!(
            SourceItem::Function(sample_function()).source_offset(),
            Some(10)
        );
        assert_eq!(
            SourceItem::Variable(SourceVariable {
                name: "x".into(),
                ty: SourceType::Int,
                source_offset: None,
            })
            .source_offset(),
            None
        );
    }

    #[test]
    fn record_opaque_detection() {
        let opaque = SourceRecord {
            kind: RecordKind::Struct,
            name: Some("opaque_ctx".into()),
            fields: None,
            source_offset: None,
        };
        assert!(opaque.is_opaque());

        let concrete = sample_record();
        assert!(!concrete.is_opaque());
    }

    #[test]
    fn field_bitfield_detection() {
        let f = SourceField {
            name: Some("flags".into()),
            ty: SourceType::UInt,
            bit_width: Some(3),
        };
        assert!(f.is_bitfield());

        let f2 = SourceField {
            name: Some("value".into()),
            ty: SourceType::Int,
            bit_width: None,
        };
        assert!(!f2.is_bitfield());
    }

    #[test]
    fn json_roundtrip_items() {
        let items: Vec<SourceItem> = vec![
            SourceItem::Function(sample_function()),
            SourceItem::Record(sample_record()),
            SourceItem::Enum(sample_enum()),
            SourceItem::TypeAlias(SourceTypeAlias {
                name: "uint32_t".into(),
                target: SourceType::UInt,
                source_offset: Some(40),
            }),
            SourceItem::Variable(SourceVariable {
                name: "errno".into(),
                ty: SourceType::Int,
                source_offset: Some(50),
            }),
            SourceItem::Unsupported(SourceUnsupported {
                name: Some("weird".into()),
                reason: "cannot model".into(),
                source_offset: None,
            }),
        ];

        for item in &items {
            let json = serde_json::to_string(item).unwrap();
            let back: SourceItem = serde_json::from_str(&json).unwrap();
            assert_eq!(*item, back);
        }
    }

    #[test]
    fn union_record() {
        let u = SourceRecord {
            kind: RecordKind::Union,
            name: Some("value".into()),
            fields: Some(vec![
                SourceField {
                    name: Some("i".into()),
                    ty: SourceType::Int,
                    bit_width: None,
                },
                SourceField {
                    name: Some("f".into()),
                    ty: SourceType::Float,
                    bit_width: None,
                },
            ]),
            source_offset: None,
        };
        assert_eq!(u.kind, RecordKind::Union);
        assert!(!u.is_opaque());
    }
}
