use std::path::Path;

use crate::scan::{scan_headers, ScanConfig};

fn corpus_root(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/corpus")
        .join(name)
}

#[test]
fn hostile_macro_env_corpus_scans_with_builtin_preprocessor() {
    let root = corpus_root("macro_env_a");
    let include_dir = root.join("include");
    let entry = root.join("entry.h");

    let result = scan_headers(
        &ScanConfig::new()
            .entry_header(&entry)
            .include_dir(&include_dir)
            .with_builtin_preprocessor(),
    )
    .expect("macro-hostile corpus should scan");

    let pkg = &result.package;
    assert!(pkg.find_type_alias("corpus_handle_t").is_some());
    assert!(pkg.find_record("corpus_config").is_some());
    assert!(pkg.find_function("corpus_open").is_some());
    assert!(pkg.find_function("corpus_format").is_some());
    assert!(result.preprocessed_source.contains("corpus_format"));
    assert!(result.preprocessed_source.contains("va_list"));
}

#[test]
fn hostile_type_env_corpus_scans_with_builtin_preprocessor() {
    let root = corpus_root("type_env_b");
    let include_dir = root.join("include");
    let entry = root.join("entry.h");

    let result = scan_headers(
        &ScanConfig::new()
            .entry_header(&entry)
            .include_dir(&include_dir)
            .with_builtin_preprocessor(),
    )
    .expect("type-hostile corpus should scan");

    let pkg = &result.package;
    assert!(pkg.find_type_alias("item_t").is_some());
    assert!(pkg.find_type_alias("item_iter_cb").is_some());
    assert!(pkg.find_record("item").is_some());
    assert!(pkg.find_record("item_bucket").is_some());
    assert!(pkg.find_function("iterate_bucket").is_some());
    assert!(result.preprocessed_source.contains("item_iter_cb"));
    assert!(result.preprocessed_source.contains("item_bucket"));
}
