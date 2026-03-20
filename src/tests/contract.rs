//! Integration tests verifying the SourcePackage contract is sufficient for downstream.

use crate::extract;
use crate::ir::*;

/// A representative C header covering all declaration families.
const REPRESENTATIVE_HEADER: &str = r#"
typedef unsigned long size_t;
typedef unsigned int uint32_t;

enum status {
    STATUS_OK = 0,
    STATUS_ERR = -1,
    STATUS_PENDING = 1
};

struct point {
    int x;
    int y;
};

struct opaque_ctx;

union value {
    int i;
    float f;
    const char *s;
};

struct nested {
    struct point pos;
    enum status state;
    unsigned flags : 4;
};

typedef void (*callback_t)(int code, const void *data);

extern int errno;

void abort(void);
int printf(const char *fmt, ...);
void *malloc(size_t size);
void free(void *ptr);
const char *strerror(int errnum);
size_t strlen(const char *s);

typedef struct point point_t;
"#;

#[test]
fn contract_covers_all_declaration_families() {
    let pkg = extract::extract_from_source(REPRESENTATIVE_HEADER).unwrap();

    // Type aliases
    assert!(pkg.find_type_alias("size_t").is_some());
    assert!(pkg.find_type_alias("uint32_t").is_some());
    assert!(pkg.find_type_alias("callback_t").is_some());
    assert!(pkg.find_type_alias("point_t").is_some());

    // Enums
    let status = pkg.find_enum("status").unwrap();
    assert_eq!(status.variants.len(), 3);
    assert_eq!(status.variants[0].name, "STATUS_OK");
    assert_eq!(status.variants[0].value, Some(0));
    assert_eq!(status.variants[1].value, Some(-1));

    // Records
    let point = pkg.find_record("point").unwrap();
    assert!(!point.is_opaque());
    assert_eq!(point.kind, RecordKind::Struct);

    let opaque = pkg.find_record("opaque_ctx").unwrap();
    assert!(opaque.is_opaque());

    let union = pkg.find_record("value").unwrap();
    assert_eq!(union.kind, RecordKind::Union);

    // Nested types
    let nested = pkg.find_record("nested").unwrap();
    let fields = nested.fields.as_ref().unwrap();
    assert_eq!(fields[0].ty, SourceType::RecordRef("point".into()));
    assert_eq!(fields[1].ty, SourceType::EnumRef("status".into()));
    assert!(fields[2].is_bitfield());

    // Functions
    assert!(pkg.find_function("abort").is_some());
    let printf = pkg.find_function("printf").unwrap();
    assert!(printf.variadic);

    let malloc = pkg.find_function("malloc").unwrap();
    assert_eq!(malloc.return_type, SourceType::ptr(SourceType::Void));

    let strerror = pkg.find_function("strerror").unwrap();
    assert_eq!(
        strerror.return_type,
        SourceType::const_ptr(SourceType::Char)
    );

    // Variables
    assert!(pkg.find_variable("errno").is_some());

    // Counts
    assert!(pkg.function_count() >= 6);
    assert!(pkg.record_count() >= 4);
    assert!(pkg.enum_count() >= 1);
    assert!(pkg.type_alias_count() >= 4);
    assert!(pkg.variable_count() >= 1);
}

#[test]
fn contract_function_pointer_typedef_is_navigable() {
    let pkg = extract::extract_from_source(REPRESENTATIVE_HEADER).unwrap();
    let cb = pkg.find_type_alias("callback_t").unwrap();
    // callback_t should be a pointer to a function
    match &cb.target {
        SourceType::Pointer { pointee, .. } => match pointee.as_ref() {
            SourceType::FunctionPointer {
                return_type,
                parameters,
                variadic,
            } => {
                assert_eq!(**return_type, SourceType::Void);
                assert_eq!(parameters.len(), 2);
                assert!(!variadic);
            }
            other => panic!("expected FunctionPointer, got {:?}", other),
        },
        other => panic!("expected Pointer, got {:?}", other),
    }
}

#[test]
fn contract_json_roundtrip_full() {
    let pkg = extract::extract_from_source(REPRESENTATIVE_HEADER).unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();
    let back: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, back);
}

#[test]
fn contract_package_contains_source_offsets() {
    let pkg = extract::extract_from_source(REPRESENTATIVE_HEADER).unwrap();
    // All items should have source offsets
    for item in &pkg.items {
        assert!(
            item.source_offset().is_some(),
            "item {:?} missing source_offset",
            item.name()
        );
    }
}

#[test]
fn contract_item_names_are_canonical() {
    let pkg = extract::extract_from_source(REPRESENTATIVE_HEADER).unwrap();

    // Function names are always present
    for f in pkg.functions() {
        assert!(!f.name.is_empty());
    }

    // Type alias names are always present
    for t in pkg.type_aliases() {
        assert!(!t.name.is_empty());
    }

    // Variable names are always present
    for v in pkg.variables() {
        assert!(!v.name.is_empty());
    }
}

#[test]
fn contract_unsupported_items_preserved() {
    let src = r#"
        static int helper(void) { return 0; }
    "#;
    let pkg = extract::extract_from_source(src).unwrap();
    // Static function should produce a diagnostic, not silently drop
    assert!(pkg.has_diagnostics());
    assert!(pkg
        .diagnostics
        .iter()
        .any(|d| d.message.contains("static")));
}

#[test]
fn contract_scan_end_to_end() {
    use crate::scan::{ScanConfig, scan_headers};

    let dir = std::env::temp_dir().join("pac_test_contract");
    let _ = std::fs::create_dir_all(&dir);

    std::fs::write(
        dir.join("api.h"),
        r#"
typedef unsigned int uint32_t;
struct config { uint32_t flags; const char *name; };
uint32_t init(struct config *cfg);
void shutdown(void);
"#,
    )
    .unwrap();

    let config = ScanConfig::new()
        .entry_header(dir.join("api.h"))
        .with_builtin_preprocessor();

    let result = scan_headers(&config).expect("scan should succeed");
    let pkg = &result.package;

    assert!(pkg.find_type_alias("uint32_t").is_some());
    assert!(pkg.find_record("config").is_some());
    assert!(pkg.find_function("init").is_some());
    assert!(pkg.find_function("shutdown").is_some());

    // Metadata
    assert!(!pkg.inputs.entry_headers.is_empty());

    let _ = std::fs::remove_dir_all(&dir);
}
