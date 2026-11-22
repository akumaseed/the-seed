//! Schema contract tests.
//!
//! - Every example in `protocol/examples/` must validate.
//! - Every fixture in `protocol/test-vectors/invalid/` must be rejected by schema
//!   validation. Fixture file names describe the rule they break.
//! - Every schema file in `protocol/json-schema/` must be registered.
//! - Rejection output must never contain the document's private values.

use std::fs;
use std::path::{Path, PathBuf};

use seed_protocol::{DocumentError, known_schemas, validate_json};

fn protocol_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../protocol")
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();
    files
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

#[test]
fn examples_are_valid() {
    let files = json_files(&protocol_dir().join("examples"));
    assert_ne!(files, Vec::<PathBuf>::new(), "no examples found");
    for path in files {
        if let Err(err) = validate_json(&read(&path)) {
            panic!("{} should be valid: {err} {err:?}", path.display());
        }
    }
}

#[test]
fn invalid_fixtures_are_rejected_by_schema_validation() {
    let files = json_files(&protocol_dir().join("test-vectors/invalid"));
    assert!(files.len() >= 20, "expected the invalid fixture set");
    for path in files {
        match validate_json(&read(&path)) {
            Ok(id) => panic!("{} was accepted as {id}", path.display()),
            Err(DocumentError::Invalid { violations, .. }) => {
                assert_ne!(violations, Vec::new(), "{}", path.display());
            }
            Err(other) => panic!(
                "{} was rejected for the wrong reason: {other:?}",
                path.display()
            ),
        }
    }
}

#[test]
fn every_schema_file_is_registered() {
    let registered: Vec<String> = known_schemas()
        .iter()
        .map(|id| format!("{}-{}.json", id.name(), id.version()))
        .collect();
    for path in json_files(&protocol_dir().join("json-schema")) {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        assert!(
            registered.contains(&name),
            "{name} exists but is not registered in seed-protocol"
        );
    }
}

#[test]
fn rejection_output_does_not_leak_private_values() {
    let path = protocol_dir()
        .join("test-vectors/invalid/continuity-manifest--inline-content-instead-of-reference.json");
    let err = validate_json(&read(&path)).unwrap_err();
    let rendered = format!(
        "{err} {err:?} {}",
        serde_json::to_string(&err.to_api_error().into_envelope()).unwrap()
    );
    assert!(!rendered.contains("secret backstory"), "{rendered}");
    if let DocumentError::Invalid { violations, .. } = err {
        assert!(
            violations
                .iter()
                .any(|v| v.instance_path == "/entries/0/content/ref")
        );
    }
}

#[test]
fn ai_suggestions_stay_non_canonical_until_approved() {
    let path = protocol_dir().join(
        "test-vectors/invalid/continuity-manifest--ai-suggestion-canonical-without-approval.json",
    );
    let Err(DocumentError::Invalid { violations, .. }) = validate_json(&read(&path)) else {
        panic!("an unapproved AI suggestion was accepted as canonical");
    };
    assert!(
        violations
            .iter()
            .any(|v| v.instance_path.starts_with("/entries/4/approval")),
        "{violations:?}"
    );
}

#[test]
fn unsupported_schema_versions_are_rejected_explicitly() {
    let files = json_files(&protocol_dir().join("test-vectors/unsupported"));
    assert_ne!(
        files,
        Vec::<PathBuf>::new(),
        "no unsupported fixtures found"
    );
    for path in files {
        assert!(
            matches!(
                validate_json(&read(&path)),
                Err(DocumentError::UnknownSchema(_))
            ),
            "{} should be rejected as an unsupported schema",
            path.display()
        );
    }
}
