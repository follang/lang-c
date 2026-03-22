//! Deterministic serialization and JSON transport tests.
//!
//! Ensures that extracting the same source always produces byte-identical JSON
//! and that the serialized form round-trips cleanly.

use crate::extract;
use crate::ir::*;
use crate::scan::{scan_headers, ScanConfig};

// --- Deterministic extraction ---

#[test]
fn determinism_simple() {
    let src = "typedef int int32_t;\nint foo(int32_t x);";
    let j1 = serde_json::to_string(&extract::extract_from_source(src).unwrap()).unwrap();
    let j2 = serde_json::to_string(&extract::extract_from_source(src).unwrap()).unwrap();
    assert_eq!(j1, j2);
}

#[test]
fn determinism_complex_header() {
    let src = r#"
typedef unsigned long size_t;
typedef unsigned int uint32_t;

enum status { OK = 0, ERR = -1 };

struct buffer {
    void *data;
    size_t size;
    size_t capacity;
};

struct buffer *buffer_new(size_t cap);
void buffer_free(struct buffer *buf);
int buffer_append(struct buffer *buf, const void *data, size_t len);

extern int debug_mode;
"#;
    let j1 = serde_json::to_string(&extract::extract_from_source(src).unwrap()).unwrap();
    let j2 = serde_json::to_string(&extract::extract_from_source(src).unwrap()).unwrap();
    assert_eq!(j1, j2);
}

#[test]
fn determinism_across_flavors_same_source() {
    // Same source, same flavor -> identical output
    let src = "struct point { int x; int y; };\nvoid draw(struct point *p);";
    let j1 = serde_json::to_string(
        &extract::parse_and_extract(src, crate::driver::Flavor::GnuC11).unwrap(),
    )
    .unwrap();
    let j2 = serde_json::to_string(
        &extract::parse_and_extract(src, crate::driver::Flavor::GnuC11).unwrap(),
    )
    .unwrap();
    assert_eq!(j1, j2);
}

#[test]
fn determinism_vendored_musl_scan() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/full_apps/external/musl/stdint");
    let include_dir = root.join("include");
    let entry = include_dir.join("stdint.h");

    let make = || {
        let result = scan_headers(
            &ScanConfig::new()
                .entry_header(&entry)
                .include_dir(&include_dir)
                .with_builtin_preprocessor(),
        )
        .expect("vendored musl stdint scan should succeed");
        serde_json::to_string(&result.package).expect("package json")
    };

    assert_eq!(make(), make());
}

#[test]
fn determinism_vendored_libpng_scan() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/full_apps/external/libpng/header");
    let include_dir = root.join("include");
    let entry = root.join("main.c");

    let make = || {
        let result = scan_headers(
            &ScanConfig::new()
                .entry_header(&entry)
                .include_dir(&include_dir)
                .with_builtin_preprocessor(),
        )
        .expect("vendored libpng scan should now succeed with builtin limits");
        serde_json::to_string(&result.package).unwrap()
    };

    assert_eq!(make(), make());
}

// --- JSON transport ---

#[test]
fn json_transport_roundtrip_with_all_item_types() {
    let src = r#"
typedef int my_int;
struct s { int x; };
union u { int i; float f; };
enum e { A = 1, B = 2 };
void func(int a, const char *b, ...);
extern int var;
"#;
    let pkg = extract::extract_from_source(src).unwrap();
    let json = serde_json::to_string(&pkg).unwrap();
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, restored);
}

#[test]
fn json_transport_pretty_roundtrip() {
    let src = "typedef void (*cb_t)(int);\nvoid set_cb(cb_t cb);\n";
    let pkg = extract::extract_from_source(src).unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, restored);
}

#[test]
fn json_transport_empty_package() {
    let pkg = SourcePackage::new();
    let json = serde_json::to_string(&pkg).unwrap();
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, restored);
}

#[test]
fn json_transport_package_with_diagnostics() {
    let src = "static int helper(void) { return 0; }\nint public_fn(void);";
    let pkg = extract::extract_from_source(src).unwrap();
    assert!(pkg.has_diagnostics());

    let json = serde_json::to_string(&pkg).unwrap();
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg.diagnostics.len(), restored.diagnostics.len());
}

#[test]
fn json_transport_builder_output() {
    let pkg = SourcePackageBuilder::new()
        .source_path("test.h")
        .item(SourceItem::Function(SourceFunction {
            name: "f".into(),
            calling_convention: CallingConvention::C,
            parameters: vec![],
            return_type: SourceType::Void,
            variadic: false,
            source_offset: None,
        }))
        .build();

    let json = serde_json::to_string(&pkg).unwrap();
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, restored);
}

// --- Schema version preserved ---

#[test]
fn json_transport_schema_version_preserved() {
    let pkg = extract::extract_from_source("int x(void);").unwrap();
    let json = serde_json::to_string(&pkg).unwrap();
    assert!(json.contains("\"schema_version\""));
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.schema_version, crate::ir::package::SCHEMA_VERSION);
}
