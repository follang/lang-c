//! Target and input provenance for the PARC frontend contract.

use serde::{Deserialize, Serialize};

/// Compiler/target identity captured alongside a source package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SourceTarget {
    #[serde(default)]
    pub target_triple: Option<String>,
    #[serde(default)]
    pub compiler_command: Option<String>,
    #[serde(default)]
    pub compiler_version: Option<String>,
    #[serde(default)]
    pub flavor: Option<String>,
}

/// One preprocessor define as provided to the frontend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDefine {
    pub name: String,
    pub value: Option<String>,
}

/// Input provenance for how a source package was produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SourceInputs {
    #[serde(default)]
    pub entry_headers: Vec<String>,
    #[serde(default)]
    pub include_dirs: Vec<String>,
    #[serde(default)]
    pub defines: Vec<SourceDefine>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_default() {
        let t = SourceTarget::default();
        assert!(t.target_triple.is_none());
        assert!(t.compiler_command.is_none());
        assert!(t.compiler_version.is_none());
        assert!(t.flavor.is_none());
    }

    #[test]
    fn target_populated() {
        let t = SourceTarget {
            target_triple: Some("x86_64-unknown-linux-gnu".into()),
            compiler_command: Some("gcc".into()),
            compiler_version: Some("13.2.0".into()),
            flavor: Some("gcc".into()),
        };
        assert_eq!(t.target_triple.as_deref(), Some("x86_64-unknown-linux-gnu"));
    }

    #[test]
    fn inputs_default() {
        let i = SourceInputs::default();
        assert!(i.entry_headers.is_empty());
        assert!(i.include_dirs.is_empty());
        assert!(i.defines.is_empty());
    }

    #[test]
    fn inputs_populated() {
        let i = SourceInputs {
            entry_headers: vec!["zlib.h".into()],
            include_dirs: vec!["/usr/include".into()],
            defines: vec![
                SourceDefine {
                    name: "DEBUG".into(),
                    value: None,
                },
                SourceDefine {
                    name: "VERSION".into(),
                    value: Some("2".into()),
                },
            ],
        };
        assert_eq!(i.entry_headers.len(), 1);
        assert_eq!(i.defines.len(), 2);
        assert!(i.defines[0].value.is_none());
        assert_eq!(i.defines[1].value.as_deref(), Some("2"));
    }

    #[test]
    fn json_roundtrip() {
        let t = SourceTarget {
            target_triple: Some("aarch64-apple-darwin".into()),
            compiler_command: Some("clang".into()),
            compiler_version: None,
            flavor: Some("clang".into()),
        };
        let json = serde_json::to_string(&t).unwrap();
        let back: SourceTarget = serde_json::from_str(&json).unwrap();
        assert_eq!(t, back);

        let i = SourceInputs {
            entry_headers: vec!["api.h".into()],
            include_dirs: vec!["/opt/include".into()],
            defines: vec![SourceDefine {
                name: "NDEBUG".into(),
                value: None,
            }],
        };
        let json = serde_json::to_string(&i).unwrap();
        let back: SourceInputs = serde_json::from_str(&json).unwrap();
        assert_eq!(i, back);
    }
}
