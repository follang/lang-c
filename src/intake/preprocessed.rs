//! Preprocessed source input builder.

use std::path::Path;

use crate::driver::Flavor;
use crate::extract::Extractor;
use crate::ir::{DiagnosticKind, SourceDiagnostic, SourcePackage};

/// A preprocessed C source ready for parsing and extraction.
#[derive(Debug, Clone)]
pub struct PreprocessedInput {
    pub source: String,
    pub source_path: Option<String>,
    pub flavor: Flavor,
}

impl PreprocessedInput {
    /// Create from a source string.
    pub fn from_string(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            source_path: None,
            flavor: Flavor::GnuC11,
        }
    }

    /// Create from a file on disk.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path)?;
        Ok(Self {
            source,
            source_path: Some(path.display().to_string()),
            flavor: Flavor::GnuC11,
        })
    }

    /// Set the source path metadata.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.source_path = Some(path.into());
        self
    }

    /// Set the parser flavor.
    pub fn with_flavor(mut self, flavor: Flavor) -> Self {
        self.flavor = flavor;
        self
    }

    /// Parse and extract, producing a `SourcePackage`.
    pub fn extract(&self) -> SourcePackage {
        let unit = match crate::parse::translation_unit(&self.source, self.flavor) {
            Ok(unit) => unit,
            Err(e) => {
                return SourcePackage {
                    source_path: self.source_path.clone(),
                    diagnostics: vec![SourceDiagnostic::error(
                        DiagnosticKind::ParseFailed,
                        format!(
                            "parse error at line {}:{}: {:?}",
                            e.line, e.column, e.expected
                        ),
                    )],
                    ..SourcePackage::new()
                };
            }
        };

        let extractor = Extractor::new();
        let (items, diagnostics) = extractor.extract(&unit);

        SourcePackage {
            source_path: self.source_path.clone(),
            items,
            diagnostics,
            ..SourcePackage::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    #[test]
    fn from_string() {
        let input = PreprocessedInput::from_string("typedef int int32_t;");
        let pkg = input.extract();
        assert_eq!(pkg.items.len(), 1);
        assert!(pkg.diagnostics.is_empty());
        assert!(pkg.source_path.is_none());
    }

    #[test]
    fn with_path() {
        let input = PreprocessedInput::from_string("void foo(void);").with_path("test.i");
        let pkg = input.extract();
        assert_eq!(pkg.source_path.as_deref(), Some("test.i"));
        assert_eq!(pkg.items.len(), 1);
    }

    #[test]
    fn parse_error() {
        let input = PreprocessedInput::from_string("this is not valid c @@@@");
        let pkg = input.extract();
        assert!(pkg.items.is_empty());
        assert!(!pkg.diagnostics.is_empty());
        assert_eq!(pkg.diagnostics[0].kind, DiagnosticKind::ParseFailed);
    }

    #[test]
    fn multiple_items() {
        let src = r#"
            typedef unsigned long size_t;
            struct point { int x; int y; };
            void *malloc(size_t n);
            void free(void *ptr);
        "#;
        let input = PreprocessedInput::from_string(src).with_path("stdlib.i");
        let pkg = input.extract();
        assert_eq!(pkg.source_path.as_deref(), Some("stdlib.i"));
        assert_eq!(pkg.items.len(), 4);
    }

    #[test]
    fn from_tempfile() {
        let dir = std::env::temp_dir().join("pac_test_intake");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_preproc.i");
        std::fs::write(&path, "extern int puts(const char *s);").unwrap();

        let input = PreprocessedInput::from_file(&path).unwrap();
        let pkg = input.extract();
        assert_eq!(pkg.items.len(), 1);
        match &pkg.items[0] {
            SourceItem::Function(f) => assert_eq!(f.name, "puts"),
            other => panic!("expected Function, got {:?}", other),
        }

        std::fs::remove_file(&path).ok();
        std::fs::remove_dir(&dir).ok();
    }

    #[test]
    fn file_not_found() {
        let result = PreprocessedInput::from_file("/nonexistent/path.i");
        assert!(result.is_err());
    }

    #[test]
    fn deterministic_output() {
        let src = "typedef int my_t; void foo(my_t x);";
        let pkg1 = PreprocessedInput::from_string(src).extract();
        let pkg2 = PreprocessedInput::from_string(src).extract();
        let j1 = serde_json::to_string(&pkg1).unwrap();
        let j2 = serde_json::to_string(&pkg2).unwrap();
        assert_eq!(j1, j2);
    }

    #[test]
    fn with_flavor() {
        let input =
            PreprocessedInput::from_string("typedef int x;").with_flavor(Flavor::StdC11);
        let pkg = input.extract();
        assert_eq!(pkg.items.len(), 1);
    }
}
